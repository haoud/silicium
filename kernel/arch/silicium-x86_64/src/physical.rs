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
    /// Map a physical frame to a virtual address
    ///
    /// # Safety
    /// The caller must ensure that the physical frame will remain valid
    /// for the lifetime of the `Mapped` object. The caller must also ensure
    /// that the frame is not already mapped to a virtual address, because it
    /// could result in mutiple mutable aliasing and undefined behavior.
    #[must_use]
    pub unsafe fn new(frame: Frame) -> Self {
        Self(Physical::from(frame))
    }

    /// Map a range of physical memory to a range of virtual memory.
    ///
    /// # Safety
    /// The caller must ensure that the physical memory will remain valid
    /// for the lifetime of the `Mapped` object. The caller must also ensure
    /// that the memory is not already mapped to a virtual address, because it
    /// could break Rust's aliasing rules and result in undefined behavior.
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
    /// The caller must ensure that the physical frame will remain valid
    /// while it is mapped to the virtual address. The caller must also ensure
    /// that the frame is not already mapped to another virtual address, because
    /// this could break the Rust aliasing rules and result in undefined behavior.
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
    /// The caller must ensure that the physical memory will remain valid
    /// while it is mapped to the virtual address. The caller must also ensure
    /// that the memory is not already mapped to another virtual address, because
    /// this could break the Rust aliasing rules and result in undefined behavior.
    #[must_use]
    pub unsafe fn leak_range(start: Physical, _len: usize) -> Virtual {
        Virtual::new_unchecked(usize::from(HHDM_START) + usize::from(start))
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
