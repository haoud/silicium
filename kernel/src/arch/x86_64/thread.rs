use crate::arch::x86_64::{
    paging::{pml4::Pml4, KERNEL_PML4},
    percpu, tss,
};
use align::Align16;
use alloc::{boxed::Box, sync::Arc};
use core::mem::MaybeUninit;
use spin::Spinlock;

core::arch::global_asm!(include_str!("asm/thread.asm"));

extern "C" {
    fn thread_switch(prev: *mut Registers, next: *const Registers);
    fn thread_jump(regs: *const Registers) -> !;
    fn thread_enter() -> !;
}

/// A thread in the system. A thread is a unit of execution that can be scheduled
/// by the kernel. Threads can be either kernel threads or user threads.
///
/// Each thread has:
/// - A kernel stack, which is used when the thread is interrupted by the kernel
///   when an interrupt occurs or during a system call. The kernel stack is also
///   used when the thread is suspended by the kernel to save a part of its state.
///
/// User threads also have:
/// - A page map level 4 table, which is used to map the virtual memory of the
///   process to the physical memory. This allows each process to have its own
///   address space. All threads of a process share the same page map level 4 table.
///   Contrary to kernel threads, user threads cannot exist without an associated
///   process.
///
/// Kernel threads do not have an associated pml4 table. This is because they all
/// share the same address space and therefore the same pml4 table [`KERNEL_PML4`].
#[derive(Debug)]
pub struct Thread {
    /// The page map level 4 table for this thread. If this thread is currently
    /// an kernel thread, this will be `None` and the thread will use the kernel
    /// page map level 4 table instead.
    pml4: Option<Arc<Spinlock<Pml4>>>,

    /// The kernel stack for this thread.
    kstack: KernelStack,
}

impl Thread {
    /// Creates a new kernel thread with the given function as the entry point.
    #[must_use]
    pub fn kernel(f: fn() -> !) -> Self {
        let mut kstack = KernelStack::new();
        kstack.write_kernel_trampoline(f as usize);
        Self { kstack, pml4: None }
    }

    /// Returns a reference to the page map level 4 table for this thread. If this
    /// thread is a kernel thread, this will return always `None`.
    #[must_use]
    pub fn pml4(&self) -> Option<&Arc<Spinlock<Pml4>>> {
        self.pml4.as_ref()
    }

    /// Returns a reference to the kernel stack for this thread.
    #[must_use]
    pub fn kstack(&self) -> &KernelStack {
        &self.kstack
    }

    /// Change the kernel stack that will be used by the thread by the given one.
    ///
    /// This function does not actually change the kernel stack used by the thread,
    /// but simply update the TSS rsp0 field (used when an user thread is interrupted
    /// by the kernel) and the percpu data kernel stack field (used when the thread
    /// makes a system call to switch to the kernel stack).
    fn change_kernel_stack(&self) {
        // SAFETY: This is safe because we provide a valid kernel stack that should
        // be big enough to be used as a kernel stack. The provided pointer also points
        // to the end of the stack as required by the functions below (because the stack
        // grows downwards on `x86_64``).
        unsafe {
            percpu::set_kernel_stack(self.kstack.bottom());
            tss::set_kernel_stack(self.kstack.bottom());
        }
    }

    /// Changes the current page map level 4 table to the one associated with
    /// this thread.
    fn change_pml4(&self) {
        // SAFETY: This is safe because all the page map level 4 table share
        // the same address space, so we can safely change the current page map
        // level 4 table to the one associated with this thread.
        unsafe {
            match self.pml4.as_ref() {
                Some(pml4) => pml4.lock().set_current(),
                None => KERNEL_PML4.set_current(),
            }
        }
    }
}

#[derive(Debug)]
pub struct KernelStack {
    /// The kernel stack. The data are uninitialized because the stack is
    /// dynamically initialized during the kernel runtime.
    data: Box<MaybeUninit<Align16<[u8; KernelStack::SIZE]>>>,

    /// The registers saved in a separate structure to make the context
    /// switching easier and the Rust code safer instead of putting this
    /// directly in the stack.
    registers: Registers,
}

impl KernelStack {
    /// The size of a kernel stack in bytes. This value should be big enough to
    /// not overflow (and remeber that the kernel is preemptible and that the
    /// stack usage can be quite high if there are too many nested interrupts),
    /// but not too big to waste memory. Since each thread has its own kernel
    /// stack, a too big value can quickly consume a lot of memory, even if the
    /// thread is really small in comparison.
    pub const SIZE: usize = 8192;

    /// Creates a kernel stack, with an pointer to an `Registers` structure
    /// with the default values and the stack pointer set to the bottom of the
    /// stack. This function also write the kernel trampoline after the bottom
    /// of the stack, needed by the [`thread_enter`] function.
    #[must_use]
    pub fn new() -> Self {
        let mut kstack = Self {
            data: Box::new_uninit(),
            registers: Registers::new(),
        };

        kstack.registers.rsp = kstack.bottom() as usize;
        kstack
    }

    /// Writes a kernel trampoline at the bottom of the stack. This function
    /// should be called when a new kernel stack is created before using it,
    /// to ensure that the `thread_enter` function will work properly.
    pub fn write_kernel_trampoline(&mut self, entry: usize) {
        self.write_trampoline(entry, self.bottom() as usize, 0x08, 0x10);
    }

    /// Writes a trampoline at the bottom of the stack. This function should be
    /// called when a new kernel stack is created before using it, to ensure that
    /// the `thread_enter` function will work properly.
    ///
    /// It will write the CS register (that will determine if the thread will be
    /// runned in user mode or kernel mode), the SS register, the RSP register that
    /// will determine the stack pointer when the thread will be runned, and the RIP
    /// register that will determine the entry point of the thread.
    pub fn write_trampoline(&mut self, entry: usize, stack: usize, cs: usize, ss: usize) {
        let rsp = self.bottom();
        // SAFETY: This is safe because the `bottom` function does not return the
        // real bottom of the stack, but the bottom of the stack minus 64 bytes,
        // that allow us to write the trampoline without writing outside of the
        // stack.
        unsafe {
            rsp.offset(0).write(entry);
            rsp.offset(1).write(cs);
            rsp.offset(2).write(0x200);
            rsp.offset(3).write(stack);
            rsp.offset(4).write(ss);
        }
    }

    /// Returns a mutable pointer to the bottom of the stack.
    #[must_use]
    pub fn bottom(&self) -> *mut usize {
        // SAFETY: This is safe because thr result is a valid pointer to the
        // end of the slice minus 64 bytes, because the [`thread_enter`] function
        // needs this space to restore an interrupt frame in order to really
        // jump to the thread entry point.
        unsafe {
            self.data
                .as_ptr()
                .cast::<usize>()
                .add(Self::SIZE - 64)
                .cast_mut()
        }
    }

    /// Returns a mutable pointer to the top of the stack.
    #[must_use]
    pub fn top(&self) -> *mut usize {
        self.data.as_ptr().cast::<usize>().cast_mut()
    }

    /// Returns a mutable pointer to the registers for this kernel stack.
    #[must_use]
    pub const fn registers(&self) -> *mut Registers {
        core::ptr::from_ref(&self.registers).cast_mut()
    }
}

impl Default for KernelStack {
    fn default() -> Self {
        Self::new()
    }
}

/// The state of a thread is saved in this structure when the thread is not
/// running. This structure is higly optimized to be as small as possible, to
/// make context switchingas fast as possible by saving as few data as possible.
///
/// In order to achieve this, the state saved are not really the state of the
/// user thread, but the state of the kernel when switching to the an another
/// thread. The real state of the thread are already saved by the interrupt
/// handler when the thread is interrupted. This allow to evict some register
/// from this structure. In addition, the `switch_context` function use advantage
/// of the fact that the system V ABI specify that some register must be saved
/// by the caller, and some other by the callee. This allow to save some
/// additional registers without having to save them manually, the compiler will
/// do it for us, but in a more efficient way.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Registers {
    pub rflags: usize,
    pub rbp: usize,
    pub rbx: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub rsp: usize,
    pub rip: usize,
}

impl Registers {
    /// Creates a new set of registers with the default values. All registers are
    /// set to 0 except the instruction pointer which is set to the address of the
    /// `thread_enter` function, which is the entry point for all created threads.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rflags: 0,
            rbp: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rsp: 0,
            rip: thread_enter as usize,
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

/// Prepare the thread to be switched to. This function should be called before
/// calling the `perform_switch` function.
/// It will change the kernel stack and the page map level 4 table to the one
/// associated with the thread.
pub fn prepare_switch(_prev: &mut Thread, next: &mut Thread) {
    next.change_kernel_stack();
    next.change_pml4();
}

/// # Important
/// When calling this function, the caller **must** ensure that no lock is held by
/// the current thread. This is because the thread will be suspended and if another
/// thread tries to acquire the lock, it will result in a deadlock that will be
/// desastrous for the system.
///
/// # Safety
/// The caller must ensure that switching threads is safe and that will not result
/// in undefined behavior or memory unsafety. The caller must also ensure that the
/// given registers pointers are valid.
pub unsafe fn perform_switch(prev: *mut Registers, next: *const Registers) {
    thread_switch(prev, next);
}

/// Prepare the thread to be jumped to. This function should be called before
/// calling the `perform_jump` function.
/// It will change the kernel stack and the page map level 4 table to the one
/// associated with the thread.
pub fn prepare_jump(thread: &mut Thread) {
    thread.change_kernel_stack();
    thread.change_pml4();
}

/// # Important
/// When calling this function, the caller **must** ensure that no lock is held by
/// the current thread. This is because the thread will be suspended and if another
/// thread tries to acquire the lock, it will result in a deadlock that will be
/// desastrous for the system.
///
/// # Safety
/// The caller must ensure that jumping to the given registers is safe and that
/// will not result in undefined behavior or memory unsafety. The caller must also
/// ensure that the given registers pointer are valid.
pub unsafe fn perform_jump(regs: *const Registers) {
    thread_jump(regs);
}
