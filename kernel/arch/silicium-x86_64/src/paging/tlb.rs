use crate::{cpu, opcode};
use addr::Virtual;

/// Invalidates the TLB entry for the given virtual address. This will
/// cause the TLB to be reloaded with the new value from the page table
/// when the address is accessed again, causing an performance hit.
/// This should be used sparingly, for exemple when an entry in the page
/// table is changed.
#[inline]
pub fn invalidate(address: Virtual) {
    opcode::invlpg(usize::from(address));
}

/// Flushes the TLB. It will invalidate all entries in the TLB, causing
/// an massive performance hit. However, entries marked as global will
/// **not** be invalidated.
/// This function should be used sparingly to avoid performance hits.
#[inline]
pub fn flush() {
    cpu::cr3::reload();
}
