pub mod cpu;
pub mod irq;
pub mod log;

/// Setup the architecture dependent parts of the kernel depending
/// on the target architecture.
#[inline]
pub fn setup() {
    #[cfg(feature = "logging")]
    log::setup();

    arch::setup();
}
