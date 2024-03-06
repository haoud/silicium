use addr::{Frame, Virtual};

/// A physical frame that can be accessed using a virtual address.
///
/// This type is used to allow reading and writing to a physical frame.
/// On some architectures that have a memory management unit (MMU) with
/// enough virtual address space, all physical memory can be mapped to
/// virtual memory. In this case, translating a physical address to a
/// virtual address is a simple addition or subtraction.
///
/// On other architectures, all the physical memory cannot be mapped to
/// virtual memory. In this case, the `Mapped` type will be used to
/// map a physical frame to a virtual address and unmap it when it is
/// no longer needed.
#[derive(Default, Debug)]
pub struct Mapped {}

impl Mapped {
    /// Map a physical frame to a virtual address.
    ///
    /// # Safety
    /// The caller must ensure that the physical frame will remain valid
    /// for the lifetime of the `Mapped` object. The caller must also ensure
    /// that the frame is not already mapped to a virtual address, because it
    /// could result in mutiple mutable aliasing and undefined behavior.
    #[must_use]
    pub unsafe fn new(_frame: Frame) -> Self {
        unimplemented!()
    }

    /// Returns the base address of the virtual address where the physical frame
    /// is mapped.
    #[must_use]
    pub fn base(&self) -> Virtual {
        unimplemented!()
    }
}

/// Unmap the physical frame from the virtual address when the `Mapped` object
/// is dropped.
impl Drop for Mapped {
    fn drop(&mut self) {
        unimplemented!()
    }
}

/// Map a physical frame to a virtual address and leak the mapping, that will
/// not automatically be unmapped. However, the caller can still manually unmap
/// the frame using the [`super::paging::unmap`] function.
///
/// # Safety
/// The caller must ensure that the physical frame will remain valid for the
/// lifetime of the mapped virtual address. The caller must also ensure that
/// the frame is not already mapped to a virtual address, because it could
/// result in mutiple mutable aliasing and undefined behavior.
#[must_use]
pub unsafe fn map_leak(_frame: Frame) -> Virtual {
    unimplemented!()
}
