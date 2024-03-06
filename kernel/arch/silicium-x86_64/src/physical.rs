use addr::{Frame, Physical, Virtual};

/// The start of the HHDM region. Since the kernel does not use the 5 level paging, the
/// HHDM region starts at `0xFFFF_8000_0000_0000`. In theory, we should use the value
/// given by Limine in the HHDM response but with the current implementation, the value
/// is always `0xFFFF_8000_0000_0000`.
const HHDM_START: Virtual = Virtual::new(0xFFFF_8000_0000_0000);

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
pub struct Mapped(Physical);

impl Mapped {
    /// Map a physical frame to a virtual address
    ///
    /// # Safety
    /// The caller must ensure that the physical frame will remain valid
    /// for the lifetime of the `Mapped` object. The caller must also ensure
    /// that the frame is not already mapped to a virtual address, because it
    /// could result in mutiple mutable aliasing and undefined behavior.
    #[must_use]
    pub unsafe fn new(frame: Frame) -> Self {
        Mapped(Physical::from(frame))
    }

    /// Map a physical address to a virtual address. Since we are using the
    /// HHDM region, we can create an [`Mapped`] object from any physical
    /// address.
    #[must_use]
    pub(crate) const unsafe fn from_physical(phys: Physical) -> Self {
        Mapped(phys)
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
