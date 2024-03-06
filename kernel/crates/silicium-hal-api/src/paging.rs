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
