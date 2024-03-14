use crate::arch;
use macros::per_cpu;

/// The preemption disable counter. This counter is per-CPU and is used to
/// disable preemption during critical sections. When the counter is 0, preemption
/// is enabled. When the counter is greater than 0, preemption is disabled.
#[per_cpu]
static mut DISABLED: usize = 0;

/// Decrement the preemption disable counter. If the counter reaches 0, preemption
/// will be re-enabled.
///
/// # Panics
/// Panics if the function is already enabled. This is a programming error meaning
/// that this function was called more times than `disable`. This is a serious
/// error and should be fixed.
pub fn enable() {
    // SAFETY: This is safe because the `DISABLED` variable is strictly per-CPU
    // and we disable interrupts when accessing it. Data races are virtually
    // impossible except if an exception occurs, but in that case, the kernel
    // is in a very bad state anyway.
    arch::irq::without(|| unsafe {
        assert!(DISABLED.get() > 0);
        DISABLED.set(DISABLED.get() - 1);
    });
}

/// Increment the preemption disable counter, disabling preemption. Preemption
/// must be re-enabled by calling `enable` the same number of times this function
/// was called.
pub fn disable() {
    // SAFETY: This is safe because the `DISABLED` variable is strictly per-CPU
    // and we disable interrupts when accessing it. Data races are virtually
    // impossible except if an exception occurs, but in that case, the kernel
    // is in a very bad state anyway.
    arch::irq::without(|| unsafe {
        DISABLED.set(DISABLED.get() + 1);
    });
}

/// Check if preemption is currently enabled.
pub fn enabled() -> bool {
    // SAFETY: This is safe because the `DISABLED` variable is strictly per-CPU
    // and we disable interrupts when accessing it. Data races are virtually
    // impossible except if an exception occurs, but in that case, the kernel
    // is in a very bad state anyway.
    arch::irq::without(|| unsafe { DISABLED.get() == 0 })
}

/// Disable preemption for the duration of the given closure.
pub fn without<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    disable();
    let ret = f();
    enable();
    ret
}
