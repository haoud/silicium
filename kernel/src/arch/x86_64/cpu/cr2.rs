use addr::Virtual;

/// Read the value of the CR2 register.
#[must_use]
pub fn read() -> Virtual {
    let value: usize;

    // SAFETY: This is safe because reading the cr2 register should not break
    // Rust's safety guarantees nor lead to undefined behavior.
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) value);
    }

    // SAFETY: The address in the cr2 register is guaranteed to be a valid
    // virtual address. We should trust the CPU, aren't we?
    unsafe { Virtual::new_unchecked(value) }
}
