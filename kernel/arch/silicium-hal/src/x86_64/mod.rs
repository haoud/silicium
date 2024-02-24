pub mod cpu;

/// Setup the architecture dependent parts of the kernel depending
/// on the target architecture.
pub fn setup() {
    arch::setup();
}
