use crate::{
    arch::x86_64::{
        apic::{self, io::IOAPIC_IRQ_BASE, local::Register},
        pit, smp,
    },
    library::seq::Seqlock,
};
use core::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use macros::init;

/// The IRQ vector used by the Local APIC timer interrupt
pub const IRQ_VECTOR: u8 = IOAPIC_IRQ_BASE;

/// The internal frequency of the Local APIC timer, in Hz
pub static INTERNAL_FREQUENCY: Seqlock<u32> = Seqlock::new(0);

/// The internal counter value of the Local APIC timer to raise an IRQ
/// at the specified frequency ([`config::TIMER_HZ`])
pub static INITIAL_COUNTER: Seqlock<u32> = Seqlock::new(0);

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
    let counter = elapsed * 100 / u32::from(config::TIMER_HZ);
    let frequency = elapsed * 100;
    let granularity = 1_000_000_000 / frequency;

    log::debug!("APIC: Calibrated Local APIC timer");
    log::debug!("APIC: Elapsed time: {} ns", frequency);
    log::debug!("APIC: Counter: {}", counter);

    // Verify that the frequency is correct
    if frequency < 25_000_000 {
        log::warn!("APIC: Internal frequency is too low ({})", frequency);
        todo!("APIC: Implement a fallback for low frequencies");
    }

    log::debug!("APIC: Internal frequency is {} MHz", frequency / 1_000_000);
    log::debug!("APIC: Timer configured at {} Hz", config::TIMER_HZ);
    log::debug!("APIC: Internal timer granularity is {} ns", granularity);

    INTERNAL_FREQUENCY.write(frequency);
    INITIAL_COUNTER.write(counter);

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
    // Enable the IRQ vector
    apic::io::enable_irq(IRQ_VECTOR);

    // Configure the Local APIC timer, respectivelly:
    // - Set the IRQ vector to 32, periodic mode
    // - Set the divide configuration to 0011 (divide by 16)
    // - Set the initial count to the computed value
    apic::local::write(Register::LVT_TIMER, u32::from(IRQ_VECTOR) | 0x20000);
    apic::local::write(Register::DIVIDE_CONFIGURATION, 0b0011);
    apic::local::write(Register::INITIAL_COUNT, INITIAL_COUNTER.read());
}

/// Prepare an IRQ to be raised in `ns` nanoseconds.
///
/// This function should not be called by the core that has called
/// [`calibrate`], since the Local APIC timer is already configured
/// to raise an IRQ at [`config::TIMER_HZ`] Hz in periodic mode, to
/// keep track of the time.
///
/// # Safety
/// The caller must ensure that raising an IRQ is safe and that the IRQ
/// vector is correctly configured in the IDT and will not lead to UB or
/// memory unsafety.
pub unsafe fn prepare_irq_in(ns: Duration) {
    let granularity = 1_000_000_000 / INTERNAL_FREQUENCY.read();
    let ns = u32::try_from(ns.as_nanos()).unwrap();

    if ns < granularity {
        log::warn!(
            "APIC: Cannot prepare an IRQ in {ns} ns, \
            granularity is {granularity} ns"
        );
        log::warn!("APIC: IRQ will be prepared in {granularity} ns instead");
    }

    apic::local::write(Register::INITIAL_COUNT, ns / granularity);
}

/// Return the internal frequency of the Local APIC timer, which is the rate
/// at the Local APIC timer counter is decremented.
#[must_use]
pub fn internal_frequency() -> u32 {
    INTERNAL_FREQUENCY.read()
}

/// Read the current value of the Local APIC timer counter. This can be useful
/// to have a precise time reference inside the current tick.
#[must_use]
pub fn internal_counter() -> u32 {
    // SAFETY: Reading the local APIC timer is safe and should not lead
    // to UB nor memory unsafety.
    unsafe { apic::local::read(Register::CURRENT_COUNT) }
}

/// Return the initial counter value of the Local APIC timer, which is the
/// value that the Local APIC timer counter is set in order to raise an IRQ
/// at the specified frequency ([`config::TIMER_HZ`]).
#[must_use]
pub fn initial_counter() -> u32 {
    INITIAL_COUNTER.read()
}

/// Check if the given IRQ is used by the Local APIC timer.
#[must_use]
pub const fn own_irq(irq: u8) -> bool {
    irq == IRQ_VECTOR
}

/// Handle the Local APIC timer interrupt.
pub fn handle_irq() {
    // The boot CPU is the only one that increments the jiffies
    // counter to keep track of the time
    if smp::is_bsp() {
        JIFFIES.fetch_add(1, Ordering::Relaxed);
    }
}
