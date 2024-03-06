use addr::{Frame, Virtual};

/// A page table is a data structure used by a virtual memory system in an
/// operating system to store the mapping between virtual addresses and physical
/// addresses. Virtual addresses are used by the CPU, and physical addresses
/// are used by the hardware to access memory. The page table is used to translate
/// virtual addresses to physical addresses.
#[derive(Default, Debug)]
pub struct PageTable {}

impl PageTable {
    /// Create a new, default page table. The created table should be able
    /// to be used as a root table directly after its creation without any
    /// further initialization.
    #[must_use]
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Load the current page table into the CPU. This function is unsafe
    /// because it can cause undefined behavior if the page table is not
    /// correctly formed.
    ///
    /// # Safety
    /// The caller must ensure that the page table is valid and correctly formed,
    /// and will remain valid and correctly formed for the duration of the
    /// execution of the code that will use the page table.
    ///
    /// The compiler cannot ensure that the table will remain while loaded into
    /// the CPU, so the caller must ensure that the table will not be deallocated
    /// while still in use.
    pub unsafe fn load_current(&mut self) {
        unimplemented!()
    }
}

bitflags::bitflags! {
    /// Flags that can be used when mapping a virtual address to a physical
    /// frame.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MapFlags: u64 {

    }

    /// Flags that can be used when mapping a virtual address to a physical
    /// frame. On some architectures, some flags may not be supported and
    /// some rights may be implied by others or being always set.
    ///
    /// For example, on x86_64, the `READ` flag is always implied if the
    /// page is present
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MapRights: u64 {
        /// Allow the user to access the memory.
        const USER = 1 << 0;

        /// Allow the memory to be read from.
        const READ = 1 << 1;

        /// Allow the memory to be written to.
        const WRITE = 1 << 2;

        /// Allow the memory to be executed.
        const EXECUTE = 1 << 3;
    }
}

/// Map the given virtual address to the given frame in the given page table with the
/// given flags and rights.
///
/// # Errors
/// - `MapError::OutOfMemory` if the kernel ran out of memory while trying to
///  allocate a new table.
/// - `MapError::AlreadyMapped` if the address is already mapped to a frame.
///
/// # Safety
/// The caller must ensure that the page table is valid and correctly formed
/// The caller must also ensure that mapping the frame to the address is safe and
/// will not cause any undefined behavior (for example, by mapping the same kernel
/// frame to two different addresses, potentially causing multiple mutable
/// aliasing or data races, potentially causing undefined behavior).
pub unsafe fn map(
    _table: &mut PageTable,
    _addr: Virtual,
    _frame: Frame,
    _flags: MapFlags,
    _rights: MapRights,
) -> Result<(), MapError> {
    unimplemented!()
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
pub unsafe fn unmap(_table: &mut PageTable, _addr: Virtual) -> Result<Frame, UnmapError> {
    unimplemented!()
}

/// Translates a virtual address to a physical frame. The virtual address is not
/// required to be page aligned, and the function will return the frame containing
/// the address if it is mapped, or `None` if it is not.
pub fn translate(_table: &mut PageTable, _addr: Virtual) -> Option<Frame> {
    unimplemented!()
}

/// Errors that can be returned when trying to map an address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MapError {
    /// The address is already mapped to a frame.
    AlreadyMapped,

    /// The kernel ran out of memory while trying to allocate a new table.
    OutOfMemory,
}

/// Errors that can be returned when trying to unmap an address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnmapError {
    /// The address is not mapped to a frame.
    NotMapped,
}
