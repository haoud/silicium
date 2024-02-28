/// Read the EFLAGS register and return its value.
#[inline]
#[must_use]
pub fn read() -> u64 {
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

/// Write the given value to the EFLAGS register.
///
/// # Safety
/// This is unsafe because modifying the EFLAGS register can have unexpected
/// side effects on the CPU and the system as a whole. For example, modifying
/// the IF (interrupt flag) bit can cause the CPU to ignore interrupts and
/// potentially hang the system, or the DF (direction flag) bit can cause string
/// instructions to operate in the opposite direction, breaking the system.
/// Therefore, the caller must ensure that the value being written is valid and
/// safe to use in his context.
#[inline]
pub unsafe fn write(eflags: u64) {
    core::arch::asm!(
        "push {0}
         popfq",
         in(reg) eflags);
}
