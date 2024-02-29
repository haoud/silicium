use addr::{Frame, Physical, Virtual};
use core::sync::atomic::{AtomicBool, Ordering};
use macros::init;
use spin::Spinlock;

/// The request that will order the Limine bootloader to provide a memory map.
static MMAP_REQUEST: Spinlock<limine::request::MemoryMapRequest> =
    Spinlock::new(limine::request::MemoryMapRequest::new());

/// A boolean that indicates whether the kernel boot memory allocator can
/// be used or not. The boot allocator can only be used after its initialization
/// and before the memory manager is initialized. See [`allocate`] for more
/// information.
/// By default, the boot allocator is enabled because it does not require any
/// initialization. The setup function simply checks if the memory map has been
/// provided by Limine.
static CAN_ALLOCATE: AtomicBool = AtomicBool::new(true);

/// Initializes the kernel boot memory allocator by requesting a memory map from
/// Limine.
///
/// # Panics
/// If Limine fails to provide a memory map, this function will panic.
#[inline]
pub fn setup() {
    assert!(
        MMAP_REQUEST.lock().get_response().is_some(),
        "Failed to get memory map from Limine"
    );
}

/// Disable the kernel boot memory allocator. This function should be called
/// before the memory manager is initialized. Calling the boot allocator after
/// it has been disabled will result in a panic.
pub fn disable_allocator() {
    CAN_ALLOCATE.store(false, Ordering::Relaxed);
}

/// Allocates a memory region of the given size during the kernel initialization,
/// when there is no memory manager available. However, the memory allocated by
/// this function cannot be freed due to the simplicity of this allocator. This
/// should not be a problem since the memory allocated during the boot process is
/// often used during the entire lifetime of the kernel.
///
/// The memory allocated is guaranteed to be aligned at least to the requested
/// alignment, which must be a power of two.
///
/// # Safety
/// This function is unsafe because it is put in the .init section and will be
/// discarded from memory after the kernel has been initialized. This means that
/// this function should only be used during the kernel initialization process.
/// Failure to do so will result in undefined behavior.
///
/// # Panics
/// This function will panic if:
/// - The bootloader has failed to provide a memory map.
/// - The alignement is not a power of two.
/// - The boot allocator has been disabled, meaning that the memory manager has
/// begun its initialization and the boot memory allocator is no longer safe to
/// use. This should never happens if the kernel is correctly implemented.
/// - There is not enough memory to allocate the requested size.
#[init]
#[must_use]
pub unsafe fn allocate_align(size: usize, align: usize) -> *mut u8 {
    Virtual::from(allocate_align_physical(size, align)).as_mut_ptr()
}

/// Allocates a memory region of the given size during the kernel initialization,
/// when there is no memory manager available. However, the memory allocated by
/// this function cannot be freed due to the simplicity of this allocator. This
/// should not be a problem since the memory allocated during the boot process is
/// often used during the entire lifetime of the kernel.
///
/// The memory allocated is guaranteed to be aligned at least to 16 bytes.
///
/// # Safety
/// This function is unsafe because it is put in the .init section and will be
/// discarded from memory after the kernel has been initialized. This means that
/// this function should only be used during the kernel initialization process.
/// Failure to do so will result in undefined behavior.
///
/// # Panics
/// This function will panic if:
/// - The bootloader has failed to provide a memory map.
/// - The boot allocator has been disabled, meaning that the memory manager has
/// begun its initialization and the boot memory allocator is no longer safe to
/// use. This should never happens if the kernel is correctly implemented.
/// - There is not enough memory to allocate the requested size.
#[init]
#[must_use]
pub unsafe fn allocate(size: usize) -> *mut u8 {
    allocate_align(size, 16)
}

/// Allocates a physical frame during the kernel initialization, when there is
/// no memory manager available. However, the memory allocated by this function
/// cannot be freed due to the simplicity of this allocator. This should not be
/// a problem since the memory allocated during the boot process is often used
/// during the entire lifetime of the kernel.
///
/// The memory allocated is guaranteed to be page aligned
///
/// # Safety
/// This function is unsafe because it is put in the .init section and will be
/// discarded from memory after the kernel has been initialized. This means that
/// this function should only be used during the kernel initialization process.
/// Failure to do so will result in undefined behavior.
///
/// # Panics
/// This function will panic if:
/// - The bootloader has failed to provide a memory map.
/// - The boot allocator has been disabled, meaning that the memory manager has
/// begun its initialization and the boot memory allocator is no longer safe to
/// use. This should never happens if the kernel is correctly implemented.
/// - There is not enough memory to allocate the requested size.
#[init]
#[must_use]
pub unsafe fn allocate_frame() -> Frame {
    Frame::from_ptr_unchecked(allocate_align_physical(4096, 4096).as_mut_ptr::<u8>())
}

/// Allocates a zeroed physical frame during the kernel initialization, when there is
/// no memory manager available. However, the memory allocated by this function
/// cannot be freed due to the simplicity of this allocator. This should not be
/// a problem since the memory allocated during the boot process is often used
/// during the entire lifetime of the kernel.
///
/// The memory allocated is guaranteed to be page aligned
///
/// # Safety
/// This function is unsafe because it is put in the .init section and will be
/// discarded from memory after the kernel has been initialized. This means that
/// this function should only be used during the kernel initialization process.
/// Failure to do so will result in undefined behavior.
///
/// # Panics
/// This function will panic if:
/// - The bootloader has failed to provide a memory map.
/// - The boot allocator has been disabled, meaning that the memory manager has
/// begun its initialization and the boot memory allocator is no longer safe to
/// use. This should never happens if the kernel is correctly implemented.
/// - There is not enough memory to allocate the requested size.
#[init]
#[must_use]
pub unsafe fn allocate_zeroed_frame() -> Frame {
    let frame = allocate_frame();
    core::ptr::write_bytes(Virtual::from(frame).as_mut_ptr::<u8>(), 0, 4096);
    frame
}

/// Allocates a memory region of the given size during the kernel initialization,
/// when there is no memory manager available. However, the memory allocated by
/// this function cannot be freed due to the simplicity of this allocator. This
/// should not be a problem since the memory allocated during the boot process is
/// often used during the entire lifetime of the kernel.
///
/// The memory allocated is guaranteed to be aligned at least to the requested
/// alignment, which must be a power of two.
///
/// # Safety
/// This function is unsafe because it is put in the .init section and will be
/// discarded from memory after the kernel has been initialized. This means that
/// this function should only be used during the kernel initialization process.
/// Failure to do so will result in undefined behavior.
///
/// # Panics
/// This function will panic if:
/// - The bootloader has failed to provide a memory map.
/// - The alignement is not a power of two.
/// - The boot allocator has been disabled, meaning that the memory manager has
/// begun its initialization and the boot memory allocator is no longer safe to
/// use. This should never happens if the kernel is correctly implemented.
/// - There is not enough memory to allocate the requested size.
#[init]
#[must_use]
pub unsafe fn allocate_align_physical(size: usize, align: usize) -> Physical {
    let mut mmap_request = MMAP_REQUEST.lock();
    let response = mmap_request
        .get_response_mut()
        .expect("Failed to get memory map from Limine");

    // Check if the alignment is a power of two
    assert!(
        align.is_power_of_two(),
        "The alignment must be a power of two"
    );

    // Check if the boot allocator has been disabled
    assert!(
        CAN_ALLOCATE.load(Ordering::Relaxed),
        "Boot allocator has been disabled"
    );

    // Search for a free region in the memory map
    let region = response
        .entries_mut()
        .iter_mut()
        .filter(|region| region.entry_type == limine::memory_map::EntryType::USABLE)
        .find(|region| region.length >= (size + align) as u64)
        .expect("Failed to find a free region in the memory map");

    // Get the base address of the region that will be used for the allocation
    // and align it to the requested alignment
    let address = usize::try_from(region.base).unwrap();

    // Calculate the offset to add to the base address to align
    // it to the requested alignment using bitwise operations
    let offset = align - (address & (align - 1));

    // Update the region's length and base address to reflect the allocation
    // and return the address of the allocated memory
    region.length -= (size + offset) as u64;
    region.base += (size + offset) as u64;

    Physical::new_unchecked(address + offset)
}
