use crate::physical;
use addr::Frame;
use macros::init;

/// Allocate a memory region using the boot allocator.
///
/// # Panics
/// Panics if the memory region cannot be allocated.
///
/// # Safety
/// The caller must ensure that this function is only used during the
/// boot process.
#[init]
#[must_use]
pub unsafe fn boot_allocate(size: usize) -> *mut u8 {
    let start = boot::allocator::allocate_align_physical(size, 16);
    let base = physical::map_leak_physical(start);
    assert!(!base.as_mut_ptr::<u8>().is_null());
    base.as_mut_ptr::<u8>()
}

/// Allocate a zeroed frame using the boot allocator.
///
/// # Panics
/// Panics if the frame cannot be allocated.
///
/// # Safety
/// The caller must ensure that this function is only used during the
/// boot process.
#[init]
#[must_use]
pub unsafe fn boot_zeroed_frame() -> Frame {
    let frame = boot::allocator::allocate_frame();
    let base = physical::map_leak(frame);

    assert!(!base.as_mut_ptr::<u8>().is_null());
    base.as_mut_ptr::<u8>().write_bytes(0, 4096);
    frame
}
