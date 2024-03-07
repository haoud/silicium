pub mod irq;
pub mod lang;
pub mod log;
pub mod paging;
pub mod percpu;
pub mod physical;

/// Request for the `HHDM` (High Half Direct Mapping) feature. This will order Limine
/// to map all physical memory to the high half of the virtual address space, at a fixed
/// offset of `0xFFFF_8000_0000_0000`. However, `Reserved` and `Bad Memory` regions will
/// not be mapped into the HHDM region.
#[used]
static HHDM_REQUEST: limine::request::HhdmRequest = limine::request::HhdmRequest::new();

/// Request for the memory map. This will order Limine to provide a memory map to the kernel.
/// The memory map is needed to initialize the memory manager and to know which memory regions
/// are usable and which are not.
#[used]
static MMAP_REQUEST: limine::request::MemoryMapRequest = limine::request::MemoryMapRequest::new();

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

    // Get the memory map from the Limine
    let limine_mmap = MMAP_REQUEST
        .get_response()
        .expect("Failed to get memory map from Limine")
        .entries();

    // Convert the memory map to the kernel's memory map format and
    // initialize the boot allocator
    let mmap = boot::mmap::from_limine(limine_mmap);
    boot::allocator::setup(mmap);

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
    let boot_allocated = boot::allocator::allocated_size();
    let mmap = boot::allocator::disable();

    boot::Info {
        mmap,
        boot_allocated,
    }
}
