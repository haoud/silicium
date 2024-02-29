use crate::{
    apic::{self, local::Register},
    pic::IRQ_BASE,
    pit,
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
    // Enable the IRQ vector
    apic::io::enable_irq(IRQ_VECTOR - IRQ_BASE);

    // Configure the Local APIC timer, respectivelly:
    // - Set the IRQ vector to 32, use periodic mode
    // - Set the divide configuration to 0011 (divide by 16)
    // - Set the initial count to the maximum value
    apic::local::write(Register::LVT_TIMER, u32::from(IRQ_VECTOR) | 0x20000);
    apic::local::write(Register::DIVIDE_CONFIGURATION, 0b0011);
    apic::local::write(Register::INITIAL_COUNT, u32::MAX);

    // Prepare and perform a 10ms sleep to calibrate the APIC timer
    pit::prepare_sleep(10);
    pit::perform_sleep();

    // Get the current count and calculate the frequency and the counter
    // to get the desired frequency (LAPIC_IRQ_HZ)
    let elapsed = u32::MAX - apic::local::read(Register::CURRENT_COUNT);
    let frequency = elapsed * 100;
    let counter = frequency / LAPIC_IRQ_HZ;

    log::debug!("APIC: Internal frequency is {frequency} Hz (calibrated with PIT)");
    log::debug!("APIC: Timer configured at {LAPIC_IRQ_HZ} Hz");

    // Configure the Local APIC timer with the calculated counter
    apic::local::write(Register::INITIAL_COUNT, counter);
}
