use arch::boot;

pub mod cpu;
pub mod irq;
pub mod log;

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
    arch::setup();
}
