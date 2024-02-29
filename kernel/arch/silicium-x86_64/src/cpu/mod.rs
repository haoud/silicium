use crate::opcode;

pub mod cr3;
pub mod eflags;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
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

    // The interrupt number and error code (if any)
    pub irq: u64,
    pub error: u64,

    // Pushed by the CPU automatically when an interrupt is triggered
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
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
