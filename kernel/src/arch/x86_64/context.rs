use crate::arch::x86_64::{percpu, tss};
use align::Align16;
use core::mem::MaybeUninit;

core::arch::global_asm!(include_str!("asm/context.asm"));

extern "C" {
    fn context_switch(prev: *mut Registers, next: *const Registers);
    fn context_jump(regs: *const Registers) -> !;
    fn context_enter() -> !;
}

#[derive(Debug)]
pub struct Context {
    /// The kernel stack for this context.
    kstack: KernelStack,

    /// The saved state of this context.
    registers: Registers,
}

impl Context {
    /// Creates a new kernel thread with the given function as the entry point.
    #[must_use]
    pub fn kernel(f: fn() -> !) -> Self {
        let mut kstack = KernelStack::kernel(f as usize);
        let registers = Registers::new(&mut kstack);

        Self { registers, kstack }
    }

    /// Returns a reference to the kernel stack for this thread.
    #[must_use]
    pub const fn kstack(&self) -> &KernelStack {
        &self.kstack
    }

    /// Returns a mutable pointer to the registers for this kernel stack.
    #[must_use]
    pub const fn registers(&self) -> *mut Registers {
        core::ptr::from_ref(&self.registers).cast_mut()
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
}

#[derive(Debug)]
pub struct KernelStack {
    /// The kernel stack. The data are uninitialized because the stack is
    /// dynamically initialized during the kernel runtime.
    data: Box<MaybeUninit<Align16<[u8; KernelStack::SIZE]>>>,
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
    /// stack, and write the kernel trampoline at the bottom of the stack that
    /// will allow the context to be resumed to the context entry point.
    #[must_use]
    pub fn kernel(entry: usize) -> Self {
        let mut kstack = Self {
            data: Box::new_uninit(),
        };

        kstack.write_kernel_trampoline(entry);
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
    /// Creates a new set of registers. All the registers are set to 0, except:
    /// - `rsp` that is set to the given kernel stack bottom,
    /// - `rip` that is set to the `context_enter` function.
    #[must_use]
    pub fn new(kstack: &mut KernelStack) -> Self {
        Self {
            rflags: 0,
            rbp: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rsp: kstack.bottom() as usize,
            rip: context_enter as usize,
        }
    }
}

/// The context to switch to another context. This structure is used to save the
/// registers of the current context and to restore the registers of the next
/// context when switching to it.
///
/// It must be acquired by calling the `prepare_switch` function before calling
/// the `perform_switch` function with the returned value.
#[derive(Debug)]
pub struct SwitchContext {
    prev: *mut Registers,
    next: *const Registers,
}

impl SwitchContext {
    /// Creates a new switch context with the given previous and next context.
    #[must_use]
    const fn new(prev: &Context, next: &Context) -> Self {
        Self {
            prev: prev.registers(),
            next: next.registers(),
        }
    }
}

/// The context to jump to another context. This structure is used restore the
/// registers of the context when jumping to it. When this structure is used
/// in conjunction with the `perform_jump` function, it will *not* save the
/// current registers, that will be lost forever.
#[derive(Debug)]
pub struct JumpContext {
    regs: *const Registers,
}

impl JumpContext {
    /// Creates a new jump context with the given context.
    #[must_use]
    const fn new(context: &Context) -> Self {
        Self {
            regs: context.registers(),
        }
    }
}

/// Prepare the thread to be switched to, and return a `SwitchContext` that
/// should be used with the `perform_switch` function to actually switch to
/// the next thread.
///
/// This function will change the kernel stack and the page map level 4 table
/// to the one associated with the next thread.
#[must_use]
pub fn prepare_switch(prev: &mut Context, next: &mut Context) -> SwitchContext {
    next.change_kernel_stack();
    SwitchContext::new(prev, next)
}

/// Perform the actual switch to the next thread. This function will save the
/// registers of the current thread and restore the registers of the next thread.
/// The `SwitchContext` must be created by calling the `prepare_switch` function
/// before calling this function.
///
/// # Important
/// When calling this function, the caller **must** ensure that no lock is held by
/// the current thread. This is because the thread will be suspended and if another
/// thread tries to acquire the lock, it will result in a deadlock that will be
/// desastrous for the system.
///
/// # Safety
/// The caller must ensure that switching threads is safe and that will not result
/// in undefined behavior or memory unsafety. The caller must also ensure that the
/// given registers pointers are valid and contain valid registers.
pub unsafe fn perform_switch(switch: SwitchContext) {
    context_switch(switch.prev, switch.next);
}

/// Prepare the thread to be jumped to, and return a `JumpContext` that should
/// be used with the `perform_jump` function to actually jump to the next thread.
///
/// This function will change the kernel stack and the page map level 4 table to
/// the one associated with the thread.
#[must_use]
pub fn prepare_jump(context: &mut Context) -> JumpContext {
    context.change_kernel_stack();
    JumpContext::new(context)
}

/// Perform the actual jump to the next thread. This function will restore the
/// registers of the thread and jump to the entry point of the thread. The
/// `JumpContext` must be created by calling the `prepare_jump` function before
/// calling this function.
///
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
pub unsafe fn perform_jump(jump: JumpContext) -> ! {
    context_jump(jump.regs);
}
