#[cfg(feature = "enforce_atomicity")]
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature = "enforce_atomicity")]
#[per_cpu]
static ATOMIC: AtomicUsize = AtomicUsize::new(0);

/// Enable the atomicity of the sleep system. This function is only for debugging
/// purposes and should not be used in production code.
///
/// If the feature `enforce_atomicity` is not enabled, this function does nothing
/// and is optimized out by the compiler.
///
/// # Panics
/// This function will panic if the atomicity is already enabled. This means that
/// the function was called more times than `disable`. This is a programming error
/// and a serious bug.
#[inline]
pub fn enable() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "enforce_atomicity")] {
            assert!(ATOMIC.local().fetch_sub(1, Ordering::Relaxed) != 0);
        }
    }
}

/// Disable the atomicity of the sleep system.
///
/// If the feature `enforce_atomicity` is not enabled, this function does nothing
/// and is optimized out by the compiler.
#[inline]
pub fn disable() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "enforce_atomicity")] {
            ATOMIC.local().fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// Check if the atomicity of the sleep system is enabled. This function is only for
/// debugging purposes and should not be used in production code.
#[cfg(feature = "enforce_atomicity")]
pub fn enabled() -> bool {
    ATOMIC.local().load(Ordering::Relaxed) == 0
}

/// Run a closure with the atomicity of the sleep system disabled. This function
/// simply executes the closure if the feature `enforce_atomicity` is not enabled.
///
/// Otherwise, this function will disable the atomicity, execute the closure and
/// then enable the atomicity again.
#[inline]
pub fn without<F: FnOnce() -> R, R>(f: F) -> R {
    disable();
    let result = f();
    enable();
    result
}
