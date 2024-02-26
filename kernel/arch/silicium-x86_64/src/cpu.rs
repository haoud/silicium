use crate::opcode;

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

/// Read the EFLAGS register and return its value.
#[inline]
#[must_use]
pub fn read_eflags() -> u64 {
    let eflags;
    // SAFETY: This is safe because we are only reading the EFLAGS register and
    // this should not have any side effects nor cause any unsafety.
    unsafe {
        core::arch::asm!(
        "pushfq
         pop {0}",
         out(reg) eflags);
    }
    eflags
}
