use crate::arch::x86_64::{
    addr::{self, Frame, Virtual},
    paging::{self, page, pml4::Pml4},
};
use hal::paging::{MapError, UnmapError};
pub use hal::paging::{MapFlags, MapRights};

/// A page table is a data structure used by a virtual memory system in an
/// operating system to store the mapping between virtual addresses and
/// physical addresses. Virtual addresses are used by the CPU, and physical
/// addresses are used by the hardware to access memory. The page table is
/// used to translate virtual addresses to physical addresses.
#[derive(Debug)]
pub struct PageTable(Pml4);

impl PageTable {
    /// Create a new, default page table. The created table should be able
    /// to be used as a root table directly after its creation without any
    /// further initialization.
    #[must_use]
    pub fn new() -> Self {
        Self(Pml4::new())
    }

    /// Load the current page table into the CPU. This function is unsafe
    /// because it can cause undefined behavior if the page table is not
    /// correctly formed.
    ///
    /// # Safety
    /// The caller must ensure that the page table is valid and correctly
    /// formed, and will remain valid and correctly formed for the duration
    /// of the execution of the code that will use the page table.
    ///
    /// The compiler cannot ensure that the table will remain while loaded
    /// into the CPU, so the caller must ensure that the table will not be
    /// deallocated while still in use.
    pub unsafe fn load_current(&mut self) {
        self.0.set_current();
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Map the given virtual address to the given frame in the given page table
/// with the given flags and rights.
///
/// # Errors
/// - `MapError::OutOfMemory` if the kernel ran out of memory while trying to
///   allocate a new table.
/// - `MapError::AlreadyMapped` if the address is already mapped to a frame.
///
/// # Safety
/// The caller must ensure that the page table is valid and correctly formed
/// The caller must also ensure that mapping the frame to the address is safe
/// and will not cause any undefined behavior (for example, by mapping the same
/// kernel frame to two different addresses, potentially causing multiple
/// mutable aliasing or data races, potentially causing undefined behavior).
pub unsafe fn map<T: addr::virt::Type>(
    table: &mut PageTable,
    addr: Virtual<T>,
    frame: Frame,
    flags: MapFlags,
    rights: MapRights,
) -> Result<(), MapError> {
    let flags = page::Flags::from(flags)
        | page::Flags::from(rights)
        | page::Flags::PRESENT;
    paging::map(&mut table.0, addr, frame, flags).map_err(|e| match e {
        paging::MapError::AlreadyMapped => MapError::AlreadyMapped,
        paging::MapError::OutOfMemory => MapError::OutOfMemory,
    })
}

/// Unmaps the given virtual address from the given page table, returning the
/// frame that was previously mapped to the address.
///
/// # Errors
/// - `UnmapError::NotMapped` if the address is not mapped to a frame.
///
/// # Safety
/// The caller must ensure that the page is valid and correctly formed.
/// The caller must also ensure that the address that will be unmapped is not
/// used anymore by the kernel. The caller is responsible for freeing (or not)
/// the frame returned by this function.
pub unsafe fn unmap<T: addr::virt::Type>(
    table: &mut PageTable,
    addr: Virtual<T>,
) -> Result<Frame, UnmapError> {
    paging::unmap(&mut table.0, addr).map_err(|e| match e {
        paging::UnmapError::NotMapped => UnmapError::NotMapped,
    })
}

/// Translates a virtual address to a physical frame. The virtual address is
/// not required to be page aligned, and the function will return the frame
/// containing the address if it is mapped, or `None` if it is not.
pub fn translate<T: addr::virt::Type>(
    table: &mut PageTable,
    addr: Virtual<T>,
) -> Option<Frame> {
    // SAFETY: This is safe since we can assume that the page table is valid
    // and correctly formed if the [`map`] and [`unmap`] functions are used
    // correctly.
    unsafe { paging::translate(&mut table.0, addr) }
}
