use super::cpu::InterruptFrame;
use core::pin::Pin;

core::arch::global_asm!(include_str!("asm/context.asm"));

extern "C" {
    fn jump_to(register: *const Registers) -> !;
}

/// The context of a user process. It contains the saved state of an user thread
/// when it is not running on the CPU.
#[derive(Debug)]
pub struct Context {
    /// The saved register state of this context.
    registers: Pin<Box<Registers>>,
}

impl Context {
    /// Create a new user context with the given entry point and stack pointer.
    #[must_use]
    pub fn new(entry: u64, stack: u64) -> Self {
        let rflags = 0x202;
        let rip = entry;
        let rsp = stack;
        let cs = 0x23; // User 64-bits code segment
        let ss = 0x2B; // User 64-bits data segment
        Self {
            registers: Box::pin(Registers {
                rflags,
                rsp,
                rip,
                cs,
                ss,
                ..InterruptFrame::default()
            }),
        }
    }
}

/// The registers of an user context are always saved when entering into
/// kernel mode. We can simply use the same structure for both.
pub type Registers = InterruptFrame;

/// The context to switch to. This structure is used to perform a context switch
/// after calling [`prepare_switch`].
#[derive(Debug)]
pub struct SwitchContext {
    next: *const Registers,
}

impl SwitchContext {
    #[must_use]
    fn new(next: &Context) -> Self {
        Self {
            next: core::ptr::from_ref(&next.registers),
        }
    }
}

/// Prepare a context switch from the current context to the next context. This will
/// save the registers of the current context into the current context and return
/// a [`SwitchContext`] that can be used to perform the actual context switch with
/// [`perform_switch`].
pub fn prepare_switch(
    register: &Registers,
    current: &mut Context,
    next: &Context,
) -> SwitchContext {
    *current.registers = register.clone();
    SwitchContext::new(next)
}

/// Switch to the next context. This function will never return and will switch
/// to the next context.
///
/// # Safety
/// The caller must ensure that the `context` is a valid [`SwitchContext`] and that
/// the current context its pointing to is still valid.
pub unsafe fn perform_switch(context: SwitchContext) -> ! {
    jump_to(context.next)
}

/// The context to jump to. This structure is used to perform a context jump
/// after calling [`prepare_jump`]. This work similarly to a context switch but
/// without saving the current context. This can be used to jump to a new context
/// without saving the current one, for example when starting the first thread on
/// a CPU or when a thread has finished.
#[derive(Debug)]
pub struct JumpContext {
    next: *const Registers,
}

impl JumpContext {
    #[must_use]
    fn new(next: &Context) -> Self {
        Self {
            next: core::ptr::from_ref(&next.registers),
        }
    }
}

/// Prepare a context jump to the next context. This will return a [`JumpContext`]
/// that can be used to perform the actual context jump with [`perform_jump`].
pub fn prepare_jump(next: &Context) -> JumpContext {
    JumpContext::new(next)
}

/// Jump to the next context. This function will never return and will jump to the
/// next context.
///
/// # Safety
/// The caller must ensure that the `context` is a valid [`JumpContext`] and that
/// the current context its pointing to is still valid.
pub unsafe fn perform_jump(context: JumpContext) -> ! {
    jump_to(context.next)
}
