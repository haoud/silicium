use core::ops::{Deref, DerefMut};

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
    /// Several conditions must be met to use this function safely:
    /// - The physical frame should not be used by the kernel.
    /// - The physical frame should not be already mapped to another virtual
    /// address.
    /// - The physical frame must remain valid until the `Mapped` object is
    /// dropped.
    #[must_use]
    pub unsafe fn new(_frame: Frame) -> Self {
        unimplemented!()
    }

    /// Map a range of physical memory to a range of virtual memory.
    ///
    /// # Safety
    /// The caller must be **extremely** careful when using this function,
    /// because it requires multiples conditions to work safely:
    /// - The physical memory range should not be used by the kernel.
    /// - The physical memory range should not be already mapped to another
    /// virtual address.
    /// - The physical memory range must remain valid until the `Mapped` object
    /// is dropped.
    ///
    /// Due to how the mapping works on many architecture, it may be possible to
    /// access physical memory that is located outside of the range because the
    /// minimal granularity of the mapping. Therefore, the caller must **not**
    /// access memory outside of the specified range. Doing so will break the
    /// guarantees of the function and result in undefined behavior.
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
    /// Several conditions must be met to use this function safely:
    /// - The physical frame should not be used by the kernel.
    /// - The physical frame should not be already mapped to another virtual
    /// address.
    /// - The physical frame must remain valid until the `Mapped` object is
    /// dropped.
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
    /// The caller must be **extremely** careful when using this function,
    /// because it requires multiples conditions to work safely:
    /// - The physical memory range should not be used by the kernel.
    /// - The physical memory range should not be already mapped to another
    /// virtual address.
    /// - The physical memory range must remain valid until the `Mapped` object
    /// is dropped.
    ///
    /// Due to how the mapping works on many architecture, it may be possible to
    /// access physical memory that is located outside of the range because the
    /// minimal granularity of the mapping. Therefore, the caller must **not**
    /// access memory outside of the specified range. Doing so will break the
    /// guarantees of the function and result in undefined behavior.
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

/// A window over a physical memory range that allows reading and writing to an
/// object of type `T`.
#[derive(Debug)]
pub struct Window<T: Sized> {
    _ptr: *mut T,
}

impl<T> Window<T> {
    /// Create a new window over the given physical memory range. The start
    /// address of the range is given by `phys` and the length of the range is
    /// specified by the size of the type `T`.
    ///
    /// # Safety
    /// This function requires the following conditions to be met to be used
    /// safely:
    /// - The physical memory range should not be used or mapped by the kernel.
    /// - The physical memory range must remain valid until the `Window` object
    /// is dropped.
    /// - The physical memory range given must be large enough to contain the
    /// object of type `T`.
    /// - The physical memory range must be properly aligned for the type `T`.
    /// - The physical memory range must be properly initialized before creating
    /// the `Window` object.
    /// - The physical memory must contain a valid object of type `T`.
    #[must_use]
    pub unsafe fn create(_phys: Physical) -> Self {
        unimplemented!()
    }
}

impl<T> Deref for Window<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl<T> DerefMut for Window<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unimplemented!()
    }
}

impl<T> Drop for Window<T> {
    fn drop(&mut self) {
        unimplemented!()
    }
}
