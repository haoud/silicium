use super::{cpu::InterruptFrame, tss};
use crate::user::thread::Trap;

core::arch::global_asm!(include_str!("asm/context.asm"));

extern "C" {
    fn execute_thread(register: &Registers);
}

/// The context of a user process. It contains the saved state of an user thread
/// when it is not running on the CPU.
#[derive(Debug)]
pub struct Context {
    /// The saved register state of this context.
    registers: Registers,
}

impl Context {
    /// Create a new user context with the given entry point and stack pointer.
    #[must_use]
    pub fn new(entry: usize, stack: usize) -> Self {
        let rflags = 0x202;
        let rip = entry as u64;
        let rsp = stack as u64;
        let cs = 0x2B; // User 64-bits code segment
        let ss = 0x23; // User 64-bits data segment
        Self {
            registers: Registers {
                rflags,
                rsp,
                rip,
                cs,
                ss,
                ..InterruptFrame::default()
            },
        }
    }

    /// Return a mutable pointer to the registers of this context. The pointer must be
    /// used with care as it is possible to corrupt the state of the context.
    ///
    /// If you use this method, you are probably doing something wrong.
    #[must_use]
    pub fn registers_ptr(&mut self) -> *mut Registers {
        core::ptr::addr_of_mut!(self.registers)
    }

    /// Return a mutable reference to the registers of this context.
    #[must_use]
    pub fn registers_mut(&mut self) -> &mut Registers {
        &mut self.registers
    }

    /// Return a reference to the registers of this context.
    #[must_use]
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    /// Return a mutable pointer to the kernel stack of this context. Silicium use
    /// a very small kernel stack for each thread that is only used when the thread
    /// enters the kernel. The kernel will save its state on this stack before
    /// switching to the per-core kernel stack. This allow to save memory when creating
    /// a kernel thread and to have a bigger kernel stack for each core that will allow
    /// use to use more stack memory and avoid stack overflow.
    #[must_use]
    pub fn kstack_rsp(&self) -> *mut usize {
        unsafe {
            core::ptr::addr_of!(self.registers)
                .byte_add(core::mem::size_of::<Registers>())
                .cast::<usize>()
                .cast_mut()
        }
    }
}

/// The registers of an user context are always saved when entering into
/// kernel mode. We can simply use the same structure for both.
pub type Registers = InterruptFrame;

/// Run the context until a trap occurs. This function will execute the user thread and
/// let it run until a trap occurs. A trap is an event that occurs during the execution
/// of the thread that requires the kernel to handle it. This can be an exception, an
/// interrupt or a system call.
///
/// This function will return the trap type that occurred and the data associated with it.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn run(context: &mut Context) -> Trap {
    // SAFETY: This is safe becayse we ensure that the kernel stack is valid and big
    // enough to handle the execution of the thread before switching to the per-core
    // kernel stack. The `execute_thread` function is safe to call but we still need
    // to put in a unsafe block because it is an external function, written in assembly.
    unsafe {
        tss::set_kernel_stack(context.kstack_rsp());
        execute_thread(&context.registers);
    }

    let registers = &context.registers;
    match registers.trap {
        0 => Trap::Exception(registers.error as usize, registers.data as u8),
        1 => Trap::Interrupt(registers.data as u8),
        2 => Trap::Syscall(registers.data as u32),
        _ => unreachable!("Unknown trap type"),
    }
}
