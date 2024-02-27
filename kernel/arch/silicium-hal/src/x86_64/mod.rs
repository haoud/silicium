use arch::boot;

pub mod log;

pub use arch::cpu;
pub use arch::irq;
pub use arch::percpu;
pub use macros::*;

/// Setup the architecture dependent parts of the kernel depending
/// on the target architecture.
#[inline]
pub fn setup() {
    // Initialize logging if this feature is enabled
    #[cfg(feature = "logging")]
    log::setup();

    // Initialize the boot allocator
    boot::setup();

    // Initialize the architecture dependent parts of the CPU
    // SAFETY: this is safe because the function is only called once
    // during boot and we initialized the boot allocator before
    // calling this function.
    unsafe {
        arch::setup();
    }
}
