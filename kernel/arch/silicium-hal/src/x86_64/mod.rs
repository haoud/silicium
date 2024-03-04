use addr::Physical;
use arrayvec::ArrayVec;

pub mod lang;
pub mod log;

pub use arch::cpu;
pub use arch::irq;
pub use arch::percpu;
pub use macros::*;

/// Setup the architecture dependent parts of the kernel depending
/// on the target architecture.
///
/// # Panics
/// This function will panic if the boot process fails
#[inline]
#[must_use]
pub fn setup() -> boot::Info {
    // Initialize logging if this feature is enabled
    #[cfg(feature = "logging")]
    log::setup();

    // Initialize the boot allocator
    arch::boot::setup();

    // Initialize the architecture dependent parts of the CPU
    // SAFETY: this is safe because the function is only called once
    // during boot and we initialized the boot allocator before
    // calling this function.
    unsafe {
        arch::setup();
    }

    // Get the memory map from the bootloader and convert it to the
    // kernel's memory map format. This is needed to support different
    // bootloaders and architectures.
    let mmap_request = arch::boot::disable_allocator();
    let mmap = mmap_request
        .get_response()
        .expect("Failed to get memory map")
        .entries()
        .iter()
        .map(|entry| boot::mmap::Entry {
            start: Physical::new(entry.base as usize),
            length: entry.length as usize,
            kind: boot::mmap::Kind::from(entry.entry_type),
        })
        .collect::<ArrayVec<boot::mmap::Entry, 32>>();

    boot::Info { mmap }
}
