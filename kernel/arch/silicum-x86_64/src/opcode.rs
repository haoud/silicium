/// Enable interrupts on the current CPU core, allowing the current workflow to be interrupted by
/// hardware interrupts. If interrupts are already enabled, this function will have no effect.
///
/// # Safety
/// This function is unsafe because this function make some assumptions about the state of the
/// system. The caller must ensure that the current CPU core is in a state where it is safe to
/// enable interrupts, and that the core can handle the interrupts that will be generated without
/// triple faulting.
/// Failing to meet these requirements can result in undefined behavior.
#[inline]
pub unsafe fn sti() {
    core::arch::asm!("sti");
}

/// Disable interrupts on the current CPU core. This will prevent the CPU from receiving any
/// interrupts until they are re-enabled. If the same interrupt is received multiple times while
/// interrupts are disabled, it will only be handled once interrupts are re-enabled.
///
/// If interrupts are already disabled, this function will have no effect.
#[inline]
pub fn cli() {
    unsafe {
        core::arch::asm!("cli");
    }
}

/// Halt the CPU until the next interrupt is received. If interrupts are disabled, this will
/// effectively halt the CPU indefinitely.
#[inline]
pub fn hlt() {
    unsafe {
        core::arch::asm!("hlt");
    }
}
