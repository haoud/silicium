use addr::Frame;

/// Read the current value of the CR3 register. This will return the current page
/// table base physical address.
#[inline]
#[must_use]
pub fn read() -> Frame {
    let cr3: usize;

    // SAFETY: Reading the CR3 register is safe and should not cause any side
    // effects that could lead to undefined behavior.
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
    }

    // SAFETY: This is safe because the address readed from the CR3 register is
    // guaranteed to be a valid physical address aligned to a 4KiB boundary.
    unsafe { Frame::new_unchecked(cr3) }
}

/// Write a new value to the CR3 register. This will update the page table base
/// physical address and cause the CPU to flush all non-global TLB entries.
///
/// # Safety
/// The caller must ensure that the new value is a valid page table base pointer
/// and is the **physical** address of the page table (and not its virtual
/// address !). Failing to do so will result in undefined behavior, likely a
/// triple fault and a system reset.
#[inline]
pub unsafe fn write(cr3: Frame) {
    core::arch::asm!("mov cr3, {}", in(reg) u64::from(cr3));
}

/// Reload the CR3 register. This will read the current value of the CR3 register
/// and write it back to the CR3 register, effectively reloading the page table
/// base pointer. This will cause the CPU too flush all non-global TLB entries.
///
/// This is safe to use because flushing the TLB has no side effects except for
/// the performance impact of flushing the TLB.
#[inline]
pub fn reload() {
    // SAFETY: This is safe because the value read from the CR3 register is
    // obviously valid and reloading the CR3 register has no side effects
    // that could lead to undefined behavior.
    unsafe {
        write(read());
    }
}
