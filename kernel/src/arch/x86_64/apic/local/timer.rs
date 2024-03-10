use crate::arch::x86_64::{
    apic::{self, io::IOAPIC_IRQ_BASE, local::Register},
    cpu::InterruptFrame,
    pit, smp,
};
use core::sync::atomic::{AtomicU64, Ordering};
use macros::init;
use seqlock::SeqLock;
use time::{frequency::Hertz32, unit::Nanosecond32};

/// The IRQ vector used by the Local APIC timer interrupt
pub const IRQ_VECTOR: u8 = IOAPIC_IRQ_BASE;

/// The internal frequency of the Local APIC timer, in Hz
pub static INTERNAL_FREQUENCY: SeqLock<Hertz32> = SeqLock::new(Hertz32::new(0));

/// The number of jiffies since the kernel has started
pub static JIFFIES: AtomicU64 = AtomicU64::new(0);

/// Initialize the Local APIC timer interrupt for the current core. This
/// function will setup the Local APIC timer to raise an IRQ at the specified
/// frequency ([`LAPIC_IRQ_HZ`]) and enable the IRQ vector ([`IRQ_VECTOR`]).
///
/// # Safety
/// The caller must ensure to only call this function once at the start
/// of the kernel, after initializing the APIC/LAPIC/IOAPIC. The core calling
/// this function should be the boot CPU.
#[init]
pub unsafe fn calibrate() {
    // Enable the IRQ vector
    apic::io::enable_irq(IRQ_VECTOR);

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
    // to get the desired frequency ([`config::TIMER_HZ`])
    let elapsed = u32::MAX - apic::local::read(Register::CURRENT_COUNT);
    let counter = (elapsed * 100) / u32::from(config::TIMER_HZ);
    let frequency = elapsed * 100;
    let granularity = 1_000_000_000 / frequency;

    // Verify that the frequency is correct
    if frequency < 25_000_000 {
        log::warn!("APIC: Internal frequency is too low ({})", frequency);
        todo!("APIC: Implement a fallback for low frequencies");
    }

    log::debug!("APIC: Internal frequency is {} MHz", frequency / 1_000_000);
    log::debug!("APIC: Timer configured at {} Hz", config::TIMER_HZ);
    log::debug!("APIC: Internal timer granularity is {} ns", granularity);
    INTERNAL_FREQUENCY.write(Hertz32::new(frequency));

    // Configure the Local APIC timer with the calculated counter
    apic::local::write(Register::INITIAL_COUNT, counter);
}

/// Initialize the Local APIC timer interrupt for the current core. This
/// will configure the Local APIC timer to raise an IRQ specified by the
/// [`IRQ_VECTOR`] in one shot mode with an divide configuration of 0b0011
/// (divide by 16).
///
/// # Safety
/// The caller must ensure to only call this function once per core during
/// the initialization of the kernel, expect for the boot CPU which should
/// call [`calibrate`] instead. This function should also be called after
/// calibrating the Local APIC timer frequency with [`calibrate`].
#[init]
pub unsafe fn setup() {
    let counter = INTERNAL_FREQUENCY.read().0 / u32::from(config::TIMER_HZ);

    // Enable the IRQ vector
    apic::io::enable_irq(IRQ_VECTOR);

    // Configure the Local APIC timer, respectivelly:
    // - Set the IRQ vector to 32, periodic mode
    // - Set the divide configuration to 0011 (divide by 16)
    // - Set the initial count to the computed value
    apic::local::write(Register::LVT_TIMER, u32::from(IRQ_VECTOR) | 0x20000);
    apic::local::write(Register::DIVIDE_CONFIGURATION, 0b0011);
    apic::local::write(Register::INITIAL_COUNT, counter);
}

/// Prepare an IRQ to be raised in `ns` nanoseconds.
///
/// This function should not be called by the core that has called [`calibrate`],
/// since the Local APIC timer is already configured to raise an IRQ at [`config::TIMER_HZ`]
/// Hz in periodic mode, to keep track of the time.
///
/// # Safety
/// The caller must ensure that raising an IRQ is safe and that the IRQ
/// vector is correctly configured in the IDT and will not lead to UB or
/// memory unsafety.
pub unsafe fn prepare_irq_in(ns: Nanosecond32) {
    let granularity = 1_000_000_000 / INTERNAL_FREQUENCY.read().0;
    let ns = ns.0;

    if ns < granularity {
        log::warn!("APIC: Cannot prepare an IRQ in {ns} ns, granularity is {granularity} ns");
        log::warn!("APIC: IRQ will be prepared in {granularity} ns");
    }

    apic::local::write(Register::INITIAL_COUNT, ns / granularity);
}

/// Check if the given IRQ is used by the Local APIC timer.
#[must_use]
pub const fn own_irq(irq: u8) -> bool {
    irq == IRQ_VECTOR
}

/// Handle the Local APIC timer interrupt.
pub fn handle_irq(_: &mut InterruptFrame) {
    // The boot CPU is the only one that increments the jiffies counter to
    // keep track of the time
    if smp::is_bsp() {
        JIFFIES.fetch_add(1, Ordering::Relaxed);
    }

    // TODO: Call the scheduler
    crate::scheduler::tick();
    // TODO: Handle timers
}