use crate::{arch::x86_64::physical, boot};
use addr::Frame;

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
    assert!(usize::from(start) != 0);
    physical::translate(start).as_mut_ptr::<u8>()
}

/// Allocate a page-aligned memory region using the boot allocator.
///
/// # Panics
/// Panics if the memory region cannot be allocated.
///
/// # Safety
/// The caller must ensure that this function is only used during the
/// boot process.
#[init]
pub unsafe fn boot_allocate_page_aligned(size: usize) -> *mut u8 {
    let start = boot::allocator::allocate_align_physical(size, 4096);
    assert!(usize::from(start) != 0);
    physical::translate(start).as_mut_ptr::<u8>()
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
    assert!(usize::from(frame) != 0);
    physical::map(frame, |virt| {
        virt.as_mut_ptr::<u8>().write_bytes(0, 4096);
        frame
    })
}
