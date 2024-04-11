use super::thread::{self, Thread};
use crate::{
    arch::{
        self,
        paging::{self, PageTable},
    },
    mm::{self, physical::allocator::Flags},
};
use addr::Virtual;
use config::PAGE_SIZE;
use core::{cmp::min, num::TryFromIntError};
use elf::{endian::NativeEndian, segment::ProgramHeader, ElfBytes};

/// Create a empty user address space and load the ELF file into it.
///
/// # Errors
/// Returns an `LoadError` if the the ELF file could not be loaded.
///
/// # Panics
/// Panics if the kernel ran out of memory when loading the ELF file or if the ELF
/// file contains overlapping segments
#[allow(clippy::cast_possible_truncation)]
pub fn load(file: &[u8]) -> Result<Thread, LoadError> {
    let elf = check_elf(ElfBytes::<NativeEndian>::minimal_parse(file)?)?;
    let mut page_table = PageTable::new();

    // Map all the segments of the ELF file that are loadable
    for phdr in elf
        .segments()
        .unwrap()
        .iter()
        .filter(|phdr| phdr.p_type == elf::abi::PT_LOAD)
    {
        let start = phdr.p_vaddr as usize;
        let size = phdr.p_memsz as usize;
        let end = start + size;

        let mapping_rights = section_paging_flags(&phdr);
        let mapping_flags = paging::MapFlags::empty();
        let start_address = Virtual::new(start);
        let end_address = Virtual::new(end);

        // Check that there is no overflow when computing the end address
        if start_address > end_address {
            return Err(LoadError::InvalidOffset);
        }

        let mut segment_offset = 0usize;
        let mut page = start_address;
        while page < end_address {
            unsafe {
                let mapping_vaddr = page;
                let mapped_frame = mm::physical::ALLOCATOR
                    .lock()
                    .allocate(Flags::empty())
                    .expect("failed to allocate frame for mapping an ELF segment");

                paging::map(
                    &mut page_table,
                    mapping_vaddr,
                    mapped_frame,
                    mapping_flags,
                    mapping_rights,
                )
                .expect("Failed to map an ELF segment");

                // The start offset of the writing in the page: it is needed to handle
                // the case where the segment is not page aligned, and therefore the
                // first page of the segment is not fully filled.
                let start_offset = usize::from(page) & 0xFFF;

                // The source address in the ELF file
                let src = file
                    .as_ptr()
                    .offset(isize::try_from(phdr.p_offset)?)
                    .offset(isize::try_from(segment_offset)?);

                // The destination address in the virtual address space (use the HHDM
                // to directly write to the physical frame)
                let dst = arch::physical::translate(mapped_frame)
                    .as_mut_ptr::<u8>()
                    .offset(isize::try_from(start_offset)?);

                // The remaning bytes to copy from the segment
                let remaning = phdr
                    .p_filesz
                    .checked_sub(segment_offset as u64)
                    .map_or(0, |v| v as usize);

                // The number of bytes to copy in this iteration: the minimum between
                // the remaining bytes to copy and the remaining bytes in the page
                // from the current start offset
                let size = min(remaning, usize::from(PAGE_SIZE) - start_offset);
                core::ptr::copy_nonoverlapping(src, dst, size);

                // Advance to the next page
                page = Virtual::new(usize::from(page) + usize::from(PAGE_SIZE)).page_align_down();
                segment_offset += size;
            }
        }
    }

    let entry = elf.ehdr.e_entry as usize;
    let stack = thread::STACK_BASE;

    for i in 1..=5 {
        let frame = mm::physical::ALLOCATOR
            .lock()
            .allocate(Flags::empty())
            .expect("failed to allocate frame for the stack");

        unsafe {
            paging::map(
                &mut page_table,
                Virtual::new(stack - i * usize::from(PAGE_SIZE)),
                frame,
                paging::MapFlags::empty(),
                paging::MapRights::USER | paging::MapRights::WRITE,
            )
            .expect("Failed to map the stack");
        };
    }

    Ok(Thread::new(entry, Arc::new(spin::Mutex::new(page_table))))
}

/// Convert the ELF flags of a section into the paging flags, used to map the
/// section with the correct permissions.
fn section_paging_flags(phdr: &ProgramHeader) -> paging::MapRights {
    let mut flags = paging::MapRights::USER;
    if phdr.p_flags & elf::abi::PF_W != 0 {
        flags |= paging::MapRights::WRITE;
    }
    if phdr.p_flags & elf::abi::PF_X != 0 {
        flags |= paging::MapRights::EXECUTE
    }
    flags
}

/// Verify that the ELF file is valid and can be run on the system.
fn check_elf(elf: ElfBytes<NativeEndian>) -> Result<ElfBytes<NativeEndian>, LoadError> {
    Ok(elf)
}

/// Error that can occur when loading an ELF file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadError {
    /// The ELF headers are invalid
    InvalidElf,

    /// The ELF file contains an invalid address (e.g. in the kernel space)
    InvalidAddress,

    /// The ELF file contains an invalid offset (e.g. an overflow when computing
    /// the end address or overlapping with kernel space)
    InvalidOffset,

    /// The ELF file contains overlapping segments
    OverlappingSegments,

    /// The ELF file is for an unsupported architecture
    UnsupportedArchitecture,

    /// The ELF file is for an unsupported endianness
    UnsupportedEndianness,
}

impl From<TryFromIntError> for LoadError {
    fn from(_: TryFromIntError) -> Self {
        LoadError::InvalidOffset
    }
}

impl From<elf::ParseError> for LoadError {
    fn from(_: elf::ParseError) -> Self {
        LoadError::InvalidElf
    }
}
