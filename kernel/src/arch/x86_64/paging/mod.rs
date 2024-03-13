use crate::arch::x86_64::{bump, cpu, physical};
use addr::{Frame, Virtual};
use macros::init;
use pml4::{MissingEntry, Pml4};

pub mod page;
pub mod pml4;
pub mod table;
pub mod tlb;

/// The kernel PML4, used to map the kernel address space. This PML4 is only modified
/// during the kernel initialization and is never modified after that. It allows us to
/// free the memory used by the bootloader and to create new PML4 extremely fast by
/// simply copying the kernel space of this PML4 into the new one.
pub static mut KERNEL_PML4: Pml4 = Pml4::empty();

/// Setup the kernel paging environment. This function recursively
/// copy the current PML4 and his subtables into the `KERNEL_PML4`
/// and reallocate every tables. This allow us to free the memory
/// used by the bootloader to store the tables after the kernel
/// initialization.
///
/// Furthermore, it preallocate all kernel PDPTs entries to simplify
/// the creation of new PML4.
///
/// # Safety
/// This function is unsafe because it must be called only once during
/// the initialization of the kernel and before initializing others cores.
#[init]
pub unsafe fn setup() {
    // Reallocate recursively all tables because there are located
    // in bootloader reclaimable memory and we will free this memory
    // after the kernel initialization. This allow us to set the `GLOBAL`
    // flags in all kernel entries, improving performances
    let current = &*physical::translate(cpu::cr3::read()).as_mut_ptr::<Pml4>();
    pml4::recursive_copy(
        KERNEL_PML4.kernel_space_mut(),
        current.kernel_space(),
        table::Level::Pml4,
    );

    // Preallocate all kernel PDPTs entries, even if they are not yet used.
    //
    // This is extremely useful because it will allow use to create new
    // pml4 extremely fast since we will juste have to copy the kernel
    // space of the `KERNEL_PML4` into the new one.
    //
    // This also solve the situation when the kernel needs to allocate
    // a new PDPT and where it must signal to all CPU that an kernel
    // PML4 entry has changed since each thread will have independant
    // PML4, even if they share the same address space
    //
    // It will simply cost use a few kilobytes of memory, but will greatly
    // improve code simplicity and performance.
    KERNEL_PML4
        .kernel_space_mut()
        .iter_mut()
        .filter(|entry| !entry.present())
        .for_each(|entry| {
            let frame = bump::boot_zeroed_frame();
            entry.add_flags(page::Flags::PRESENT | page::Flags::WRITABLE | page::Flags::GLOBAL);
            entry.set_address(frame);
        });
}

/// Load the kernel PML4 into the CR3 register.
///
/// # Safety
/// This function is unsafe because it must be called only once per core
/// and only during the initialization of the kernel, and after the [`setup`]
/// function has been called. Failure to do so will result in undefined behavior
/// likely to cause a triple fault and a system reboot.
#[init]
pub unsafe fn load_kernel_pml4() {
    KERNEL_PML4.set_current();
}

/// Map the given virtual address to the given frame in the given PML4, with the
/// given flags.
///
/// # Errors
/// - `MapError::OutOfMemory` if the kernel ran out of memory while trying to
///  allocate a new page table.
/// - `MapError::AlreadyMapped` if the address is already mapped to a frame.
///
/// # Safety
/// The caller must ensure that the PML4 is valid and that each frame pointing to
/// a table is correctly allocated and initialized and belongs to the PML4.
/// The caller must also ensure that mapping the frame to the address is safe and
/// will not cause any undefined behavior (for example, by mapping the same kernel
/// frame to two different addresses, potentially causing multiple multable aliasing
/// or data races).
/// Failure to do so will result in undefined behavior.
pub unsafe fn map(
    pml4: &mut Pml4,
    addr: Virtual,
    frame: Frame,
    flags: page::Flags,
) -> Result<(), MapError> {
    let entry = pml4
        .fetch_last_entry(addr, MissingEntry::Allocate(flags))
        .map_err(|_| MapError::OutOfMemory)?;

    if entry.present() {
        log::warn!("Attempt to map an already mapped address: {:#x}", addr);
        return Err(MapError::AlreadyMapped);
    }

    entry.set_address(frame);
    entry.add_flags(flags);
    Ok(())
}

/// Unmaps the given virtual address from the given PML4, returning the frame
/// that was previously mapped to the address.
///
/// # Errors
/// - `UnmapError::NotMapped` if the address is not mapped to a frame.
///
/// # Safety
/// The caller must ensure that the PML4 is valid and that each frame pointing to
/// a table is correctly allocated and initialized and belongs to the PML4.
/// The caller must also ensure that the address that will be unmapped is not
/// used anymore. The caller is responsible for freeing (or not) the frame
/// returned by this function.
/// Failure to do so will result in undefined behavior.
pub unsafe fn unmap(pml4: &mut Pml4, addr: Virtual) -> Result<Frame, UnmapError> {
    let entry = pml4
        .fetch_last_entry(addr, MissingEntry::Fail)
        .map_err(|_| UnmapError::NotMapped)?;

    if let Some(frame) = entry.address() {
        entry.clear();
        tlb::shootdown(addr);
        Ok(frame)
    } else {
        log::warn!("Attempt to unmap an already unmapped address: {:#x}", addr);
        Err(UnmapError::NotMapped)
    }
}

/// Translates a virtual address to a physical frame. The virtual address is not
/// required to be page aligned, and the function will return the frame containing
/// the address if it is mapped, or `None` if it is not.
///
/// # Safety
/// The caller must ensure that the PML4 is valid and that each frame pointing to
/// a table is correctly allocated and initialized and belongs to the PML4.
/// Failure to do so will result in undefined behavior, likely a page fault.
#[must_use]
pub unsafe fn translate(pml4: &mut Pml4, addr: Virtual) -> Option<Frame> {
    pml4.fetch_last_entry(addr, MissingEntry::Fail)
        .map(|entry| entry.address())
        .ok()?
}

/// Error returned when trying to map an address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MapError {
    /// The kernel ran out of memory while trying to allocate a new page table.
    OutOfMemory,

    /// The address is already mapped to a frame.
    AlreadyMapped,
}

/// Error returned when trying to unmap an address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnmapError {
    /// The address is not mapped to a frame.
    NotMapped,
}
