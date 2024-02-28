use addr::Frame;

bitflags::bitflags! {
    /// Represents the flags of a page table entry. See Intel Vol. 3A, Section 4.5 for more
    /// information about page tables and their flags.
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct Flags: u64 {
        /// If set, the page is present in memory. Otherwise, the page is not present, and
        /// the bits 12-51 of the entry are ignored and free to use for other purposes.
        const PRESENT = 1 << 0;

        /// If set, the page is writable. Otherwise, the page is read-only. If the
        /// write protection bit of the CR0 register is not set, this flag is ignored
        /// in supervisor mode.
        const WRITABLE = 1 << 1;

        /// If set, the page is accessible from user mode. Otherwise, the page is only
        /// accessible from kernel mode.
        const USER = 1 << 2;

        /// If set, the page caching strategy is set to write-through. Otherwise,
        /// the caching strategy is set to write-back. This is useful for memory-mapped I/O.
        const WRITE_THROUGH = 1 << 3;

        /// If set, the page is not cached. Otherwise, the page is cached according
        /// to the caching strategy set by the `WRITE_THROUGH` flag.
        const NO_CACHE = 1 << 4;

        /// If set, the page has been accessed. When the page is accessed, the flag is
        /// set by the processor (but never cleared by the it). This flag can also be
        /// set manually.
        const ACCESSED = 1 << 5;

        /// If set, the page has been written to. When the page is written to, the flag
        /// is set by the processor (but never cleared by it). This flag can also be set
        /// manually.
        const DIRTY = 1 << 6;

        /// If set, the page is a huge page. This flags is only valid for PDPT entries and
        /// PD entries. If the flags is set to a PD entry, the entry maps directly to a
        /// 2 MiB page (and the address must be aligned to 2 MiB too). If the flag is set
        /// to a PDPT entry, the entry maps to a 1 GiB page (and the address must be aligned
        /// to 1 GiB too).
        const HUGE_PAGE = 1 << 7;

        /// If set, the page is global. A global page is not flushed from the TLB when
        /// CR3 is modified. This is often used for kernel pages, and can improves
        /// performance.
        const GLOBAL = 1 << 8;

        const BIT_9  = 1 << 9;
        const BIT_10 = 1 << 10;
        const BIT_11 = 1 << 11;
        const BIT_52 = 1 << 52;
        const BIT_53 = 1 << 53;
        const BIT_54 = 1 << 54;
        const BIT_55 = 1 << 55;
        const BIT_56 = 1 << 56;
        const BIT_57 = 1 << 57;
        const BIT_58 = 1 << 58;
        const BIT_59 = 1 << 59;
        const BIT_60 = 1 << 60;
        const BIT_61 = 1 << 61;
        const BIT_62 = 1 << 62;

        /// If set, the page is not executable. By default, all pages are executable.
        /// This flag is only valid if the `NXE` bit of the `EFER` register is set,
        /// otherwise it is ignored.
        const NO_EXECUTE = 1 << 63;
    }
}

/// Represents a page entry
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Entry(u64);

impl Entry {
    /// The mask to apply to the entry to get the address of the page. The last 12 bits
    /// of the entry and the first 12 bits of the address are used for flags by the CPU.
    pub const ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;

    /// An empty page table entry. The entry is not present in memory, and the address is
    /// set to 0.
    pub const EMPTY: Self = Self(0);

    /// Create a new page table entry with the given address and flags. The address must
    /// be page aligned, because the last 12 bits are used for flags by the CPU.
    #[must_use]
    pub fn new(frame: Frame, flags: Flags) -> Self {
        Self((u64::from(frame) & Self::ADDRESS_MASK) | flags.bits())
    }

    /// Create an empty page table entry. The entry is not present in memory, and the
    /// address is set to 0.
    #[must_use]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    /// Set the address of the page table entry. This function does not modify any flags
    /// of the entry.
    pub fn set_address(&mut self, addr: Frame) {
        self.0 = (self.flags().bits()) | (u64::from(addr) & Self::ADDRESS_MASK);
    }

    /// Clear the given flags of the page table entry. This function does not modify the
    /// address of the entry, and simply clears the given flags of the entry.
    pub fn remove_flags(&mut self, flags: Flags) {
        self.0 &= !flags.bits();
    }

    /// Add the given flags to the page table entry. This function does not modify the
    /// address of the entry, and simply adds the given flags to the entry.
    pub fn add_flags(&mut self, flags: Flags) {
        self.0 |= flags.bits();
    }

    /// Returns `true` if the page is present in memory, `false` otherwise.
    #[must_use]
    pub const fn present(&self) -> bool {
        self.flags().contains(Flags::PRESENT)
    }

    /// Returns `true` if the entry is a huge page, `false` otherwise. The size of the
    /// page depends on the level of the page table:
    ///  - If the entry is a PD entry, the page is 2 MiB.
    ///  - If the entry is a PDPT entry, the page is 1 GiB.
    /// Calling this function on a PML4 entry or a PT entry will not return a meaningful
    /// result.
    #[must_use]
    pub const fn huge_page(&self) -> bool {
        self.flags().contains(Flags::HUGE_PAGE)
    }

    /// Returns `true` if the page is executable, `false` otherwise.
    #[must_use]
    pub const fn executable(&self) -> bool {
        !self.flags().contains(Flags::NO_EXECUTE)
    }

    /// Returns `true` if the page is writable, `false` otherwise.
    #[must_use]
    pub const fn writable(&self) -> bool {
        self.flags().contains(Flags::WRITABLE)
    }

    /// Returns `true` if the page is dirty, `false` otherwise. A page is dirty
    /// if it has been written to, or if the flag has been set manually.
    #[must_use]
    pub const fn dirty(&self) -> bool {
        self.flags().contains(Flags::DIRTY)
    }

    /// Returns `true` if the page has been accessed, `false` otherwise. A page
    /// is accessed if it has been read from, or if the flag has been set manually.
    #[must_use]
    pub const fn accessed(&self) -> bool {
        self.flags().contains(Flags::ACCESSED)
    }

    /// Returns `true` if the page not user accessible, `false` otherwise.
    #[must_use]
    pub const fn kernel(&self) -> bool {
        !self.user()
    }

    /// Returns `true` if the page is user accessible, `false` otherwise.
    #[must_use]
    pub const fn user(&self) -> bool {
        self.flags().contains(Flags::USER)
    }

    /// Set the entry to 0, indicating that the page is not present in memory.
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// Returns the flags of this entry.
    #[must_use]
    pub const fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.0)
    }

    /// Returns the physical address of the page mapped by this entry. If the entry is
    /// not present, `None` is returned.
    #[must_use]
    pub const fn address(&self) -> Option<Frame> {
        if self.flags().contains(Flags::PRESENT) {
            // SAFETY: This is safe because the address in the entry is guaranteed to be
            // page aligned and to be a valid physical address (less than 2^52) because
            // there is simply not enough bits to represent a larger address in the
            // entry.
            unsafe { Some(Frame::new_unchecked((self.0 & Self::ADDRESS_MASK) as usize)) }
        } else {
            None
        }
    }
}
