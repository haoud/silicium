use crate::arch::x86_64::opcode;
pub use cpuid::cpuid;

pub mod cpuid;
pub mod cr0;
pub mod cr2;
pub mod cr3;
pub mod cr4;
pub mod rflags;
pub mod xcr0;

/// The interrupt frame that is pushed to the stack when an interrupt is triggered.
/// This structure is used to save the state of the CPU before the interrupt
/// handler is called.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[repr(C, align(16))]
pub struct InterruptFrame {
    // Preserved registers
    pub rbp: u64,
    pub rbx: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,

    // Scratched registers
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,

    pub padding: u64,

    /// Custom data pushed by the interrupt handler. This data is used to pass
    /// additional information to the interrupt handler. For example, the IRQ
    /// number for an interrupt is pushed in this field.
    pub data: u64,

    /// The trap type. For now, there is only 3 types of traps:
    /// - Exception
    /// - Interrupt
    /// - System call
    pub trap: u64,

    /// The error code. It is either pushed by the CPU automatically when certain
    /// exceptions are triggered or pushed by the interrupt handler. In the last
    /// case, the error code is set to 0.
    pub error: u64,

    // Pushed by the CPU automatically when an interrupt is triggered
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

/// Return an unique identifier for the current CPU core. This identifier is
/// unique for each core and is used to identify the core in the SMP.
#[must_use]
pub fn id() -> u64 {
    let id: u64;
    // SAFETY: This is safe because the gs points to the per-cpu data, and gs:8
    // contains the lapic_id of the current core
    unsafe {
        core::arch::asm!("mov {}, gs:8", out(reg) id);
    }
    id
}

/// Halt the current CPU core indefinitely. This function is used to permanently
/// stop the CPU core from executing any further instructions and put it into a
/// low-power state.
/// This action is irreversible and the only way to recover from it is to reset
/// the entire system.
#[inline]
pub fn halt() -> ! {
    loop {
        opcode::cli();
        opcode::hlt();
    }
}

/// Enable the required CPU features that are needed for the kernel to work
/// properly. If a feature is not supported by the CPU, the kernel will panic
/// or triple fault.
///
/// # Safety
/// The caller must ensure to call this function only once per core, only during
/// the kernel initialization phase, and as soon as possible on boot to avoid
/// any issues.
#[init]
pub unsafe fn enable_required_features() {
    cr4::enable(cr4::Features::PGE);
    log::trace!("CR4 enabled features: {:?}", cr4::read());
}
