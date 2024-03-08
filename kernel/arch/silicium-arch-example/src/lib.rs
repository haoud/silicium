#![cfg_attr(not(test), no_std)]
#![feature(negative_impls)]
use macros::init;

pub mod irq;
#[cfg(not(test))]
pub mod lang;
pub mod log;
pub mod paging;
pub mod percpu;
pub mod physical;

/// Initialize the architecture specific requirements. This function will be
/// calling very early during the initialization of the kernel.lmù
/// However, the logging system is still available (if enabled) and the boot
/// allocator is active at this point to allocate memory.
///
/// # Safety
/// This function should only be called once by the boot processor during the
/// initialization of the kernel. Calling this function more than once or after
/// the initialization of the kernel will result in undefined behavior.
#[init]
#[allow(clippy::missing_panics_doc)]
pub unsafe fn setup() {
    panic!("This architecture is not supported by the kernel")
}
