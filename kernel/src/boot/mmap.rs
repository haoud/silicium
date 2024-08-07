use crate::arch::x86_64::addr::Physical;
use arrayvec::ArrayVec;
use limine::memory_map::EntryType;

/// A memory map entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    /// The start address of the memory region
    pub start: Physical,

    /// The length of the memory region
    pub length: usize,

    /// The kind of memory region
    pub kind: Kind,
}

impl Entry {
    /// Returns the end address of the memory region (excluding the returned
    /// address)
    ///
    /// # Panics
    /// Panics if the end address is not representable as a `Physical` address
    #[must_use]
    pub fn end(&self) -> Physical {
        self.start + self.length
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            start: Physical::new(0),
            kind: Kind::Reserved,
            length: 0,
        }
    }
}

/// The kind of memory region. This is used to determine what the memory
/// region is used for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Usable memory, available for allocation
    Usable,

    /// Memory that is currently used by the kernel
    Kernel,

    /// Memory that is reserved by the hardware
    Reserved,

    /// Memory that is used by the ACPI tables and can be reclaimed
    AcpiReclaimable,

    /// Memory that is used by the bootloader and can be reclaimed
    BootloaderReclaimable,

    /// Memory that is defective and should not be used
    BadMemory,
}

impl Kind {
    /// Returns `true` if the memory region is regular memory, and `false`
    /// otherwise. Regular memory is memory that is available for allocation
    /// and is not reserved by the hardware:
    /// - [`Kind::Usable`]
    /// - [`Kind::Kernel`]
    /// - [`Kind::AcpiReclaimable`]
    /// - [`Kind::BootloaderReclaimable`]
    #[must_use]
    pub const fn regular_memory(self) -> bool {
        matches!(
            self,
            Self::Usable
                | Self::Kernel
                | Self::AcpiReclaimable
                | Self::BootloaderReclaimable
        )
    }

    /// Returns `true` if the memory region belongs to the kernel, and `false`
    /// otherwise. Kernel memory is memory that is used by the kernel and is
    /// not available for allocation. This includes the kernel itself and any
    /// modules that are loaded by the kernel.
    pub const fn is_kernel(self) -> bool {
        matches!(self, Self::Kernel)
    }
}

impl From<limine::memory_map::EntryType> for Kind {
    fn from(entry_type: limine::memory_map::EntryType) -> Self {
        match entry_type {
            EntryType::BOOTLOADER_RECLAIMABLE => Self::BootloaderReclaimable,
            EntryType::ACPI_RECLAIMABLE => Self::AcpiReclaimable,
            EntryType::KERNEL_AND_MODULES => Self::Kernel,
            EntryType::BAD_MEMORY => Self::BadMemory,
            EntryType::USABLE => Self::Usable,
            _ => Self::Reserved,
        }
    }
}

/// # Panics
/// Panics if the memory map has more than 32 entries and cannot fit into an
/// [`ArrayVec`].
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn from_limine(mmap: &[&limine::memory_map::Entry]) -> ArrayVec<Entry, 32> {
    mmap.iter()
        .map(|entry| Entry {
            start: Physical::new(entry.base as usize),
            length: entry.length as usize,
            kind: Kind::from(entry.entry_type),
        })
        .collect::<ArrayVec<Entry, 32>>()
}
