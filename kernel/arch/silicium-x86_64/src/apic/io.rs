use crate::apic;
use addr::Virtual;

/// The base address of the IOAPIC MMIO
pub const IOAPIC_BASE: Virtual = Virtual::new(0xFFFF_8000_FEC0_0000);

/// The base IRQ number for the IOAPIC
pub const IOAPIC_IRQ_BASE: u8 = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(u32);

impl Register {
    pub const ID: Register = Register(0x00);
    pub const VERSION: Register = Register(0x01);
    pub const ARBITRATION_ID: Register = Register(0x02);
    pub const REDIRECTION_TABLE_BASE: Register = Register(0x10);

    #[must_use]
    pub const fn redirection_low(n: u8) -> Register {
        Register(Self::REDIRECTION_TABLE_BASE.0 + (n as u32) * 2)
    }

    /// Compute the register address for the given redirection entry. The result
    /// will always to read the high 32 bits of the entry.
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
    let count = (((read(Register::VERSION) >> 16) + 1) & 0xFF) as u8;
    log::debug!("IOAPIC has {} entries", count);

    // Disable all interrupts
    for i in 0..count {
        write(Register::redirection_high(i), 0);
        write(Register::redirection_low(i), 1 << 16);
    }

    // Send EOI to the local APIC in case there are any pending interrupts
    apic::local::end_of_interrupt();
}

/// Enable an IRQ in the IOAPIC.
///
/// # Safety
/// This function is unsafe because enabling an IRQ can cause undefined
/// behavior if the IOAPIC is not properly initialized and mapped, or if
/// the IDT handler is misconfigured. The caller must ensure that enabling
/// the IRQ is safe and will not cause undefined behavior.
pub unsafe fn enable_irq(irq: u8) {
    let count = (((read(Register::VERSION) >> 16) + 1) & 0xFF) as u8;
    let vector = u32::from(IOAPIC_IRQ_BASE + irq);

    if irq >= count {
        log::warn!(
            "IOAPIC: Trying to enable IRQ {} out of range (max {})",
            irq,
            count
        );
        return;
    }

    // Enable the IRQ by setting the vector and unmasking it
    write(Register::redirection_high(irq), 0);
    write(Register::redirection_low(irq), vector);
}

/// Disable an IRQ in the IOAPIC.
///
/// # Safety
/// This function is unsafe because disabling an IRQ can cause undefined
/// behavior if the IOAPIC is not properly initialized and mapped. The caller
/// must ensure that disabling the IRQ is safe and will not cause undefined
/// behavior.
pub unsafe fn disable_irq(irq: u8) {
    let count = (((read(Register::VERSION) >> 16) + 1) & 0xFF) as u8;

    if irq >= count {
        log::warn!(
            "IOAPIC: Trying to disable IRQ {} out of range (max {})",
            irq,
            count
        );
        return;
    }

    // Disable the IRQ by masking it
    write(Register::redirection_high(irq), 0);
    write(Register::redirection_low(irq), 1 << 16);
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
    IOAPIC_BASE.as_mut_ptr::<u32>().write_volatile(reg.0);
    // Read the value from IOWIN
    IOAPIC_BASE.as_ptr::<u32>().byte_add(0x10).read_volatile()
}
