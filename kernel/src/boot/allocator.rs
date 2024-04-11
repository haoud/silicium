use crate::boot::mmap;
use addr::{Frame, Physical};
use arrayvec::ArrayVec;
use config::PAGE_SIZE;
use core::sync::atomic::{AtomicUsize, Ordering};
use macros::init;

/// The memory map provided by the bootloader.
static MMAP: spin::Mutex<Option<ArrayVec<mmap::Entry, 32>>> = spin::Mutex::new(None);

/// The total amount of memory allocated by the boot allocator.
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

/// Initializes the kernel boot memory allocator with the given memory map request.
///
/// # Panics
/// If Limine fails to provide a memory map, this function will panic.
#[inline]
pub fn setup(mmap: ArrayVec<mmap::Entry, 32>) {
    MMAP.lock().replace(mmap);
}

/// Disable the kernel boot memory allocator and return the memory map request. The memory map
/// is sanitized to ensure that all usable memory regions base address are aligned to the page size.
///
/// This function should be called before the memory manager is initialized. Calling the
/// boot allocator after it has been disabled will result in a panic.
///
/// # Panics
/// This function will panic if the boot allocator has already been disabled.
pub fn disable() -> ArrayVec<mmap::Entry, 32> {
    let mut mmap = MMAP
        .lock()
        .take()
        .expect("Boot allocator has been disabled");

    // Align all usable memory regions to the page size
    mmap.iter_mut()
        .filter(|region| region.kind == mmap::Kind::Usable)
        .filter(|region| !region.start.is_page_aligned())
        .for_each(|region| {
            let offset = usize::from(region.start) % usize::from(PAGE_SIZE);
            let correction = usize::from(PAGE_SIZE) - offset;

            region.start = Physical::new(usize::from(region.start) - correction);
            region.length -= correction;
        });
    mmap
}

/// Returns the total amount of memory allocated by the boot allocator.
#[must_use]
pub fn allocated_size() -> usize {
    ALLOCATED.load(Ordering::Relaxed)
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
    let mut mmap = MMAP.lock();
    let mmap = mmap
        .as_mut()
        .expect("Boot allocator has not been initialized");

    // Check if the alignment is a power of two
    assert!(
        align.is_power_of_two(),
        "The alignment must be a power of two"
    );

    // Search for a free region in the memory map
    let region = mmap
        .iter_mut()
        .filter(|region| region.kind == mmap::Kind::Usable)
        .find(|region| region.length >= size + align)
        .expect("Failed to find a free region in the memory map");

    // Get the base address of the region that will be used for the allocation
    // and align it to the requested alignment
    let address = usize::from(region.start);

    // Calculate the offset to add to the base address to align
    // it to the requested alignment using bitwise operations
    let offset = (align - (address & (align - 1))) & (align - 1);

    // Update the region's length and base address to reflect the allocation
    // and return the address of the allocated memory
    region.start = Physical::new(address + offset + size);
    region.length -= offset + size;

    ALLOCATED.fetch_add(size + offset, Ordering::Relaxed);
    Physical::new_unchecked(address + offset)
}
