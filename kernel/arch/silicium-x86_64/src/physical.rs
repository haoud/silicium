use core::ops::{Deref, DerefMut};

use addr::{Frame, Physical, Virtual};

/// The start of the HHDM region. Since the kernel does not use the 5 level paging, the
/// HHDM region starts at `0xFFFF_8000_0000_0000`. In theory, we should use the value
/// given by Limine in the HHDM response but with the current implementation, the value
/// is always `0xFFFF_8000_0000_0000`.
const HHDM_START: Virtual = Virtual::new(0xFFFF_8000_0000_0000);

/// A window that allows reading and writing to a physical frame.
/// On this architecture, all physical memory can be mapped to virtual memory,
/// and therefore, the `AccessWindow` type can simply be implemented with
/// a simple addition of the physical address to the [`HHDM_START`] address.
#[derive(Default, Debug)]
pub struct AccessWindow(Physical);

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
    pub unsafe fn new(frame: Frame) -> Self {
        Self(Physical::from(frame))
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
    pub unsafe fn range(start: Physical, _len: usize) -> Self {
        Self(start)
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
    pub unsafe fn leak(frame: Frame) -> Virtual {
        Virtual::new_unchecked(usize::from(HHDM_START) + usize::from(frame))
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
    pub unsafe fn leak_range(start: Physical, _len: usize) -> Virtual {
        // SAFETY: This is safe since the HHDM_START is a valid canonical address, and in the
        // `x86_64` architecture, the physical address is at most 52 bits. Therefore, the
        // addition of the physical address to the HHDM_START will always result in a valid
        // canonical address.
        unsafe { Virtual::new_unchecked(usize::from(HHDM_START) + usize::from(start)) }
    }

    /// Returns the base address of the virtual address where the physical frame
    /// is mapped.
    #[must_use]
    pub fn base(&self) -> Virtual {
        // SAFETY: This is safe since the HHDM_START is a valid canonical address, and in the
        // `x86_64` architecture, the physical address is at most 52 bits. Therefore, the
        // addition of the physical address to the HHDM_START will always result in a valid
        // canonical address.
        unsafe { Virtual::new_unchecked(usize::from(HHDM_START) + usize::from(self.0)) }
    }
}

/// A window over a physical memory range that allows reading and writing to an
/// object of type `T`.
#[derive(Debug)]
pub struct Window<T: Sized> {
    ptr: *mut T,
}

impl<T> Window<T> {
    /// Create a new window over the given physical memory range. The start
    /// address of the range is given by `phys` and the length of the range is
    /// specified by the size of the type `T`.
    ///
    /// ***Please, please, read the safety section before using this function. There
    /// are many conditions that must be met to use this function safely, and
    /// failing to meet them all will result in undefined behavior that will
    /// break the kernel in an horrible way !***
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
    pub unsafe fn create(phys: Physical) -> Self {
        // SAFETY: This is safe since the HHDM_START is a valid canonical address, and in the
        // `x86_64` architecture, the physical address is at most 52 bits. Therefore, the
        // addition of the physical address to the HHDM_START will always result in a valid
        // canonical address.
        let ptr = (usize::from(HHDM_START) + usize::from(phys)) as *mut T;
        Self { ptr }
    }
}

impl<T> Deref for Window<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe since the pointer is valid and should points
        // to a valid object of type `T`, properly initialized and aligned.
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Window<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: This is safe since the pointer is valid and should points
        // to a valid object of type `T`, properly initialized and aligned.
        unsafe { &mut *self.ptr }
    }
}
