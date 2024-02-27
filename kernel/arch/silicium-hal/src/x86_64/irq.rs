/// Enable interrupts on the current core.
///
/// # Safety
/// This function is unsafe because enabling interrupts, contrary to disabling
/// them, can lead to unexpected behavior, memory unsafety, data races and
/// other issues if not used correctly.
/// To correctly use this function, it is required to ensure that the caller
/// has correctly set up the interrupt. Then, the caller must ensure that the
/// portion of code that is executed with interrupts enabled is safe to be
/// executed concurrently with other code that may also have interrupts
///
/// Even if this doesn't lead to memory unsafety, it can still lead to deadlocks
/// if the code that is executed with interrupts enabled is not reentrant and
/// is called from a context where interrupts are disabled.
#[inline]
pub unsafe fn enable() {
    arch::opcode::sti();
}

/// Disable interrupts on the current core.
#[inline]
pub fn disable() {
    arch::opcode::cli();
}

/// Check if interrupts are enabled. Returns true if interrupts are enabled,
/// false otherwise.
#[inline]
#[must_use]
pub fn enabled() -> bool {
    let eflags = arch::cpu::read_eflags();
    eflags & (1 << 9) == 0
}

/// Execute a closure with interrupts disabled. After the closure is executed,
/// the previous state of the interrupts is restored.
pub fn without<T, F: FnOnce() -> T>(f: F) -> T {
    let enabled = enabled();
    disable();
    let object = f();
    if enabled {
        // SAFETY: Enabling interrupts is safe in this contexte because they
        // were enabled before calling this function and we simply restore the
        // previous state. This is safe because the caller is responsible for
        // managing its own code, and are not our problem here. If the code was
        // unsound before calling this function, we can't do anything about it.
        unsafe {
            enable();
        }
    }
    object
}