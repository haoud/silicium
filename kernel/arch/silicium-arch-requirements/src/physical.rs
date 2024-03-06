use addr::{Frame, Virtual};
use core::ops::{Deref, DerefMut};

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
    /// Map a physical frame to a virtual address
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
}

/// Allow reading from the virtual address
impl Deref for Mapped {
    type Target = Virtual;
    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

/// Allow writing to the virtual address
impl DerefMut for Mapped {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
