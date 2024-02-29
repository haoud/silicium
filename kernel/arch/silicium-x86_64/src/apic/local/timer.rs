use crate::{
    apic::{self, local::Register},
    pic::IRQ_BASE,
};
use macros::init;

/// The frequency of IRQ raised by the Local APIC timer interrupt
pub const LAPIC_IRQ_HZ: u32 = 1000;

/// The IRQ vector used by the Local APIC timer interrupt
pub const IRQ_VECTOR: u8 = 32;

/// Initialize the Local APIC timer interrupt for the current core. This
/// function will setup the Local APIC timer to raise an IRQ at the specified
/// frequency ([`LAPIC_IRQ_HZ`]) and enable the IRQ vector ([`IRQ_VECTOR`]).
///
/// # Safety
/// The caller must ensure to only call this function once at the start
/// of the kernel, after initializing the APIC/LAPIC/IOAPIC.
#[init]
pub unsafe fn setup() {
    let count = 1_000_000;
    // Enable the IRQ vector
    apic::io::enable_irq(IRQ_VECTOR - IRQ_BASE);

    // Set the divisor to 16, configure the timer to periodic mode and set the IRQ vector.
    apic::local::write(Register::LVT_TIMER, u32::from(IRQ_VECTOR) | 0x20000);
    apic::local::write(Register::DIVIDE_CONFIGURATION, 0b1011);
    apic::local::write(Register::INITIAL_COUNT, count);
}
