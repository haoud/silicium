use super::page;
use crate::physical;
use addr::Frame;
use core::ops::{Deref, DerefMut};

/// A page table, which is a collection of page table entries. A page table
/// contains 512 entries, each of which is 8 bytes in size. The page table
/// must be aligned to a 4096-byte boundary, and is used to map virtual
/// addresses to physical addresses.
#[derive(Debug)]
#[repr(C, align(4096))]
pub struct Table([page::Entry; Table::COUNT]);

impl Table {
    /// The number of entries in the page table
    pub const COUNT: usize = 512;

    /// Create a new empty page table, with all entries set to empty.
    #[must_use]
    pub const fn empty() -> Self {
        Self([page::Entry::empty(); Self::COUNT])
    }

    /// Returns a mutable pointer to the page table from the given frame.
    #[must_use]
    pub fn from_frame_mut(frame: Frame) -> *mut Self {
        // SAFETY: The mapping to the frame will remain valid for the lifetime
        // of the kernel
        unsafe { physical::Mapped::new(frame).base().as_mut_ptr::<Self>() }
    }

    /// Returns a pointer to the page table from the given frame.
    #[must_use]
    pub fn from_frame(frame: Frame) -> *const Self {
        // SAFETY: The mapping to the frame will remain valid for the lifetime
        // of the kernel
        unsafe { physical::Mapped::new(frame).base().as_ptr::<Self>() }
    }
}

/// The page table is not `Unpin` because it when it is loaded into the CR3
/// register, it is not allowed to move in memory.
impl !Unpin for Table {}

impl Deref for Table {
    type Target = [page::Entry; Table::COUNT];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::empty()
    }
}

/// The level of the page table hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// The page map level 4
    Pml4,
    /// The page directory pointer table
    Pdpt,
    /// The page directory
    Pd,
    /// The page table
    Pt,
}

impl Level {
    /// Get the next level in the page table hierarchy. If the current level is
    /// the page table, then there is no next level and `None` is returned.
    #[must_use]
    pub const fn next(&self) -> Option<Self> {
        match self {
            Self::Pml4 => Some(Self::Pdpt),
            Self::Pdpt => Some(Self::Pd),
            Self::Pd => Some(Self::Pt),
            Self::Pt => None,
        }
    }
}
