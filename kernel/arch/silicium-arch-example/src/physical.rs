use addr::{Frame, Physical, Virtual};

/// A window that allows reading and writing to a physical frame.
///
/// On some architectures that have a memory management unit (MMU) with
/// enough virtual address space, all physical memory can be mapped to
/// virtual memory. In this case, the `AccessWindow` type can simply be
/// implemented with arithmetic operations on the physical address.
///
/// On other architectures, all the physical memory cannot be mapped to
/// virtual memory. In this case, this type will need to temporarily map
/// the physical memory to virtual memory in order to access it.
#[derive(Default, Debug)]
pub struct AccessWindow {}

impl AccessWindow {
    /// Map a physical frame to a virtual address.
    ///
    /// # Safety
    /// The caller must ensure that the physical frame will remain valid
    /// for the lifetime of the `Mapped` object. The caller must also ensure
    /// that the frame is not already mapped to a virtual address, because it
    /// could break Rust's aliasing rules and result in undefined behavior.
    #[must_use]
    pub unsafe fn new(_frame: Frame) -> Self {
        unimplemented!()
    }

    /// Map a range of physical memory to a range of virtual memory.
    ///
    /// # Safety
    /// The caller must ensure that the physical memory will remain valid
    /// for the lifetime of the `Mapped` object. The caller must also ensure
    /// that the memory is not already mapped to a virtual address, because it
    /// could break Rust's aliasing rules and result in undefined behavior.
    #[must_use]
    pub unsafe fn range(_start: Physical, _len: usize) -> Self {
        unimplemented!()
    }

    /// Map a physical frame to a virtual address and leak the mapping by
    /// returning the virtual address.
    ///
    /// However, the caller can still reclaim the frame by calling manually
    /// calling [`crate::paging::unmap`] on the returned virtual address.
    ///
    /// # Safety
    /// The caller must ensure that the physical frame will remain valid
    /// while it is mapped to the virtual address. The caller must also ensure
    /// that the frame is not already mapped to another virtual address, because
    /// this could break the Rust aliasing rules and result in undefined behavior.
    #[must_use]
    pub unsafe fn leak(_frame: Frame) -> Virtual {
        unimplemented!()
    }

    /// Map a range of physical memory to a range of virtual memory and leak the
    /// mapping by returning the virtual address.
    ///
    /// However, the caller can still reclaim the memory by calling manually
    /// calling [`crate::paging::unmap`] on the returned virtual address.
    ///
    /// # Safety
    /// The caller must ensure that the physical memory will remain valid
    /// while it is mapped to the virtual address. The caller must also ensure
    /// that the memory is not already mapped to another virtual address, because
    /// this could break the Rust aliasing rules and result in undefined behavior.
    #[must_use]
    pub unsafe fn leak_range(_start: Physical, _len: usize) -> Virtual {
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
impl Drop for AccessWindow {
    fn drop(&mut self) {
        unimplemented!()
    }
}
