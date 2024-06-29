use crate::arch::x86_64::{
    addr::Virtual,
    apic::{
        self,
        local::{IpiDestination, IpiPriority},
    },
    cpu, opcode,
};

/// The vector number for the TLB shootdown interrupt.
pub const SHOOTDOWN_VECTOR: u8 = 0xA0;

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

/// Returns true if the given interrupt vector is owned by the TLB module.
#[inline]
pub fn own_irq(irq: u8) -> bool {
    irq == SHOOTDOWN_VECTOR
}

/// Shootdown the TLB on all CPU cores. This will cause the TLB to be
/// entirely invalidated on all cores, causing a massive performance
/// hit. This should be used sparingly.
///
/// # Improvements
/// - Only invalidate the TLB entries that are needed.
/// - Only send an IPI to the cores that need to invalidate their TLB.
/// - Lazy TLB invalidation.
pub fn shootdown(address: Virtual) {
    // SAFETY: This is safe because send an IPI to others core should be safe
    // since we can assume that the IDT is correctly initialized and that the
    // interrupt will not cause UB or memory unsafety.
    unsafe {
        apic::local::send_ipi(
            IpiDestination::AllExcludingSelf,
            IpiPriority::Fixed,
            SHOOTDOWN_VECTOR,
        );
    }

    // Flush the TLB on the current CPU.
    invalidate(address);
}
