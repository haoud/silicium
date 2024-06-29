use crate::arch::x86_64::{addr::Virtual, apic};

/// The base address of the IOAPIC MMIO
pub const IOAPIC_BASE: Virtual = Virtual::new(0xFFFF_8000_FEC0_0000);

/// The base IRQ number for the IOAPIC
pub const IOAPIC_IRQ_BASE: u8 = 32;

/// The number of IRQs in the IOAPIC
static mut IRQ_COUNT: u8 = 0;

/// A register in the IOAPIC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(u32);

impl Register {
    pub const ID: Register = Register(0x00);
    pub const VERSION: Register = Register(0x01);
    pub const ARBITRATION_ID: Register = Register(0x02);
    pub const REDIRECTION_TABLE_BASE: Register = Register(0x10);

    /// Compute the register address for the given redirection entry.
    /// The result will always to read the low 32 bits of the entry.
    #[must_use]
    pub const fn redirection_low(n: u8) -> Register {
        Register(Self::REDIRECTION_TABLE_BASE.0 + (n as u32) * 2)
    }

    /// Compute the register address for the given redirection entry.
    /// The result will always to read the high 32 bits of the entry.
    #[must_use]
    pub const fn redirection_high(n: u8) -> Register {
        Register(Self::REDIRECTION_TABLE_BASE.0 + (n as u32) * 2 + 1)
    }
}

/// Setup the IOAPIC and disable all interrupts
///
/// # Safety
/// This function is unsafe because it must only be called once, before
/// initializing other cores and after the Local APIC was initialized.
/// It should also only be called during the kernel initialization.
pub unsafe fn setup() {
    IRQ_COUNT = (((read(Register::VERSION) >> 16) + 1) & 0xFF) as u8;
    log::debug!("IOAPIC: {} entries found", IRQ_COUNT);

    // Disable all interrupts
    for i in 0..IRQ_COUNT {
        write(Register::redirection_high(i), 0);
        write(Register::redirection_low(i), 1 << 16);
    }

    // Send EOI to the local APIC in case there are any pending interrupts
    apic::local::end_of_interrupt();
}

/// Enable an IRQ in the IOAPIC, identified by its vector.
///
/// # Safety
/// This function is unsafe because enabling an IRQ can cause undefined
/// behavior if the IOAPIC is not properly initialized and mapped, or if
/// the IDT handler is misconfigured. The caller must ensure that enabling
/// the IRQ is safe and will not cause undefined behavior.
pub unsafe fn enable_irq(vector: u8) {
    if !is_irq(vector) {
        log::warn!(
            "IOAPIC: Trying to enable IRQ {vector} not owned by the IOAPIC"
        );
        return;
    }

    let irq = vector - IOAPIC_IRQ_BASE;

    // Enable the IRQ by setting the vector and unmasking it
    write(Register::redirection_high(irq), 0);
    write(Register::redirection_low(irq), u32::from(vector));
}

/// Disable an IRQ in the IOAPIC, identified by its vector.
///
/// # Safety
/// This function is unsafe because disabling an IRQ can cause undefined
/// behavior if the IOAPIC is not properly initialized and mapped. The caller
/// must ensure that disabling the IRQ is safe and will not cause undefined
/// behavior.
pub unsafe fn disable_irq(vector: u8) {
    if !is_irq(vector) {
        log::warn!(
            "IOAPIC: Trying to disable IRQ {vector} not owned by the IOAPIC"
        );
        return;
    }

    // Disable the IRQ by masking it
    let irq = vector - IOAPIC_IRQ_BASE;
    write(Register::redirection_high(irq), 0);
    write(Register::redirection_low(irq), 1 << 16);
}

/// Return the number of IRQs in the IOAPIC. If the IOAPIC is not initialized,
/// this will return 0.
#[must_use]
pub fn entry_count() -> u8 {
    // SAFETY: IRQ_COUNT is set in `setup` and never modified afterwards,
    // and we don't creat mutable references to it, so it's safe to read it.
    unsafe { IRQ_COUNT }
}

/// Check if an interrupt is an IRQ from the IOAPIC
#[must_use]
pub fn is_irq(vector: u8) -> bool {
    entry_count() != 0
        && vector >= IOAPIC_IRQ_BASE
        && vector < IOAPIC_IRQ_BASE + entry_count()
}

/// Write a value to a register in the IOAPIC
///
/// # Safety
/// This function is unsafe because writing to the IOAPIC can cause
/// undefined behavior or memory unsafety if the IOAPIC is not properly
/// initialized and mapped. Furthermore, writing to the a register can
/// lead to undefined behavior depending on the register and value.
pub unsafe fn write(reg: Register, value: u32) {
    // Tell IOREGSEL what register we want to write to
    IOAPIC_BASE.as_mut_ptr::<u32>().write_volatile(reg.0);
    // Write the value to IOWIN
    IOAPIC_BASE
        .as_mut_ptr::<u32>()
        .byte_add(0x10)
        .write_volatile(value);
}

/// Read a register from the IOAPIC
///
/// # Safety
/// This function is unsafe because reading from the IOAPIC can cause
/// undefined behavior or memory unsafety if the IOAPIC is not properly
/// initialized and mapped.
#[must_use]
pub unsafe fn read(reg: Register) -> u32 {
    // Tell IOREGSEL what register we want to read from
    // And then read the value from IOWIN
    IOAPIC_BASE.as_mut_ptr::<u32>().write_volatile(reg.0);
    IOAPIC_BASE.as_ptr::<u32>().byte_add(0x10).read_volatile()
}
