use super::{
    page,
    table::{self, Table},
    translate,
};
use crate::{boot, cpu};
use addr::{Frame, Virtual};
use core::pin::Pin;
use macros::init;
use tailcall::tailcall;

/// The page map level 4 table. This table is the root of the page table hierarchy
/// and is used to translate virtual addresses to physical addresses. The PML4 table
/// contains 512 entries, each entry points to a page directory pointer table (PDPT).
///
/// This structure also contains the physical address of the PML4 table, cached to
/// avoid translating the virtual address of the table each time the table is loaded
/// into the CR3 register.
#[derive(Debug)]
pub struct Pml4 {
    /// The page map level 4 table
    table: Pin<Table>,

    /// The physical address of the page table. This is used to load the page table
    /// into the CR3 register. If this is `None`, the physical address of the page
    /// table is not cached and must be translated when needed.
    frame: Option<Frame>,
}

impl Pml4 {
    /// Create a new empty root page table, with all entries set to empty. Loading
    /// this page table into the CR3 register will result in a page fault that will
    /// lead to a double and triple fault because there is no translation for any
    /// virtual address.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            table: Pin::new(Table::empty()),
            frame: None,
        }
    }

    /// Fetch the page table entry for the given virtual address.
    ///
    /// If an entry is missing during the translation, `behavior` will determine
    /// what to do:
    /// - `MissingEntry::Fail`: Stop the translation and return an error
    /// - `MissingEntry::Allocate(flags)`: Allocate a new frame for the missing table,
    /// update the entry with the new frame address and flags and continue the translation.
    ///
    /// # Errors
    /// - `FetchError::MissingTable`: A table is missing and the behavior is `Fail`
    /// - `FetchError::OutOfMemory`: A table is missing and the behavior is `Allocate`, but
    ///  the kernel is out of memory and cannot allocate a new frame
    ///
    /// # Safety
    /// The caller must ensure that the table is correctly initialized and that every
    /// entries are valid (with a correct physical address) and accessible. The caller
    /// must have an exclusive access to all the table that are referenced by the different
    /// levels of tables.
    /// Furthermore, if the caller specifies the `Allocate` behavior, the caller must ensure
    /// that the frame allocator is correctly initialized and that it is safe to allocate a
    /// new frame.
    /// Failing to do so will result in undefined behavior.
    pub unsafe fn fetch_last_entry(
        &mut self,
        addr: Virtual,
        behavior: MissingEntry,
    ) -> Result<&mut page::Entry, FetchError> {
        Self::fetch_entry(&mut self.table[..], addr, table::Level::Pml4, behavior)
    }

    #[tailcall]
    unsafe fn fetch_entry(
        table: &mut [page::Entry],
        addr: Virtual,
        level: table::Level,
        behavior: MissingEntry,
    ) -> Result<&mut page::Entry, FetchError> {
        // Get the index of the entry in the table depending on the current level
        let index = match level {
            table::Level::Pml4 => (usize::from(addr) >> 39) & 0x1FF,
            table::Level::Pdpt => (usize::from(addr) >> 30) & 0x1FF,
            table::Level::Pd => (usize::from(addr) >> 21) & 0x1FF,
            table::Level::Pt => (usize::from(addr) >> 12) & 0x1FF,
        };

        let entry = &mut table[index];

        if let Some(next) = level.next() {
            let frame = match entry.address() {
                Some(frame) => frame,
                None => match behavior {
                    MissingEntry::Fail => return Err(FetchError::MissingTable),
                    MissingEntry::Allocate(flags) => {
                        // TODO: Allocate a new zeroed frame
                        let frame = Frame::try_new(0).ok_or(FetchError::OutOfMemory)?;

                        // Update the entry with the new frame address and flags and continue
                        // the translation with the new table
                        entry.set_address(frame);
                        entry.add_flags(flags);
                        frame
                    }
                },
            };

            // Get the next table and continue the translation
            let table = &mut *Table::from_frame_mut(frame);
            Self::fetch_entry(&mut table[..], addr, next, behavior)
        } else {
            Ok(entry)
        }
    }

    /// Set the page table as the current one. This will load the page table into
    /// the CR3 register and flush all the old TLB entries exvept the global ones.
    ///
    /// # Safety
    /// This function is unsafe because the caller must ensure that the page table
    /// is accessible and correctly initialized. Failure to do so will result in a
    /// page fault that will probably lead to a double and triple fault and a
    /// system reset.
    ///
    /// # Panics
    /// Panic if this function cannot translate the virtual address of the table
    /// into a physical address. This should never happen and is probably a bug in
    /// the kernel.
    pub unsafe fn set_current(&mut self) {
        // If the physical address of the page table is not already cached,
        // translate it and cache it
        if self.frame.is_none() {
            self.frame = Some(
                translate(self, Virtual::from_ptr_unchecked(self.table.as_ptr()))
                    .expect("Failed to translate the PML4 virtual address"),
            );
        }

        // Load the page table into the CR3 register
        cpu::cr3::write(self.frame.unwrap_unchecked());
    }

    /// Returns a mutable pointer to the current page table.
    #[must_use]
    pub fn get_current_mut() -> *mut Self {
        let current = Virtual::from(cpu::cr3::read());
        current.as_mut_ptr::<Self>()
    }

    /// Returns a pointer to the current page table.
    #[must_use]
    pub fn get_current() -> *const Self {
        let current = Virtual::from(cpu::cr3::read());
        current.as_ptr::<Self>()
    }

    /// Returns a mutable slice of the page table entries. The slice contains all
    /// the PML4 entries.
    #[must_use]
    pub fn address_space_mut(&mut self) -> &mut [page::Entry] {
        &mut self.table[..]
    }

    /// Returns a slice of the page table entries. The slice contains all the PML4
    /// entries.
    #[must_use]
    pub fn address_space(&self) -> &[page::Entry] {
        &self.table[..]
    }

    /// Returns a mutable slice of the kernel space page table entries. The slice
    /// contains the page table directory entries dedicated to kernel space memory.
    /// The last 256 entries are dedicated to kernel space memory (`0xFFFF_8000_0000_0000`
    /// to `0xFFFF_FFFF_FFFF_FFFF`).
    #[must_use]
    pub fn kernel_space_mut(&mut self) -> &mut [page::Entry] {
        &mut self.table[256..512]
    }

    /// Returns a mutable slice of the user space page table entries. The slice contains
    /// the page table directory entries dedicated to user space memory. The first 256
    /// entries are dedicated to user space memory (`0x0000_0000_0000_0000` to
    /// `0x0000_7FFF_FFFF_FFFF`).
    #[must_use]
    pub fn user_space_mut(&mut self) -> &mut [page::Entry] {
        &mut self.table[0..256]
    }

    /// Returns a slice of the kernel space page table entries. The slice contains the
    /// page table directory entries dedicated to kernel space memory. The last 256
    /// entries are dedicated to kernel space memory (`0xFFFF_8000_0000_0000` to
    /// `0xFFFF_FFFF_FFFF_FFFF`).
    #[must_use]
    pub fn kernel_space(&self) -> &[page::Entry] {
        &self.table[256..512]
    }

    /// Returns a slice of the user space page table entries. The slice contains the
    /// page table directory entries dedicated to user space memory. The first 256
    /// entries are dedicated to user space memory (`0x0000_0000_0000_0000` to
    /// `0x0000_7FFF_FFFF_FFFF`).
    #[must_use]
    pub fn user_space(&self) -> &[page::Entry] {
        &self.table[0..256]
    }
}

/// Recursively copy all the entries from the source table to the destination table.
/// The destination table should be empty because if the destination table or its
/// children already contains page entries, they are simply overwritten and the old
/// frames are not freed.
///
/// # Safety
/// This function is unsafe because it must be called only once during the initialization
/// of the kernel and before initializing others cores.
#[init]
pub unsafe fn recursive_copy(to: &mut [page::Entry], from: &[page::Entry], level: table::Level) {
    for (to, from) in to
        .iter_mut()
        .zip(from.iter())
        .filter(|(_, entry)| entry.present())
    {
        // Copy the entry from the source table to the destination table
        *to = *from;

        // If the level has a next level, recursively copy the next level
        if let Some(next) = level.next() {
            // If the level is PDPT or PD and the huge page flag is set, we can
            // simply skip next levels because the huge page flag means that
            // the next levels are not used and the current entry is the last
            // one and directly map a 1GiB or 2MiB page.
            match level {
                table::Level::Pdpt | table::Level::Pd => {
                    if from.huge_page() {
                        return;
                    }
                }
                _ => {}
            }

            // Get the physical address of the source table and allocate a new
            // frame for the destination table
            let src_frame = from.address().unwrap_unchecked();
            let dst_frame = boot::allocate_frame();

            // Copy the source table into the destination table
            let dst = Virtual::from(dst_frame).as_mut_ptr::<page::Entry>();
            let src = Virtual::from(src_frame).as_ptr::<page::Entry>();
            core::ptr::copy_nonoverlapping(src, dst, Table::COUNT);

            // Update the destination entry with the new frame address
            to.add_flags(page::Flags::GLOBAL);
            to.set_address(dst_frame);

            // Recursively copy the next level
            let to = Table::from_frame_mut(dst_frame);
            let from = Table::from_frame(src_frame);
            recursive_copy(&mut (*to)[..], &(*from)[..], next);
        }
    }
}

/// A enumeration of possible behavior when a page table entry is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingEntry {
    /// Allocate a new frame and continue the translation
    Allocate(page::Flags),

    /// Stop the translation and return an error
    Fail,
}

/// A enumeration of possible errors when fetching an page table entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchError {
    /// A table is missing and the behavior is fail
    MissingTable,

    /// A table is missing and the behavior is allocate, but the
    /// kernel is out of memory
    OutOfMemory,
}
