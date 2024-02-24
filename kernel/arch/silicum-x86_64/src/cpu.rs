use crate::opcode;

/// Halt the current CPU core indefinitely. This function is used to permanently
/// stop the CPU core from executing any further instructions and put it into a
/// low-power state.
/// This action is irreversible and the only way to recover from it is to reset
/// the entire system.
#[cold]
pub fn halt() -> ! {
    loop {
        opcode::cli();
        opcode::hlt();
    }
}
