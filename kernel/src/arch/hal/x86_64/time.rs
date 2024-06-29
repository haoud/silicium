use crate::arch::x86_64::apic;
use config::TIMER_HZ;
use core::{sync::atomic::Ordering, time::Duration};

/// Returns the number of jiffies since the kernel has started.
///
/// A jiffy is a unit of time used in the kernel. It is defined as
/// the number of ticks that the kernel has been running.
#[must_use]
pub fn get_jiffies() -> u64 {
    apic::local::timer::JIFFIES.load(Ordering::Relaxed)
}

/// Returns the frequency of the jiffies in hertz, which is the number of
/// jiffies per second.
pub const fn jiffies_frequency() -> u64 {
    TIMER_HZ as u64
}

/// Returns the granularity of the jiffies in milliseconds, which is the
/// time between each jiffy.
#[must_use]
pub const fn jiffies_granularity() -> Duration {
    Duration::from_millis(1_000 / TIMER_HZ as u64)
}

/// Returns the offset in nanoseconds from the last jiffy. This is useful
/// to have a precise time when the jiffy occurred.
#[must_use]
pub fn jiffies_nano_offset() -> Duration {
    let frequency = apic::local::timer::internal_frequency();
    let counter = apic::local::timer::internal_counter();
    let initial = apic::local::timer::initial_counter();
    let granularity = 1_000_000_000 / u64::from(frequency);

    let elapsed = initial - counter;
    Duration::from_nanos(u64::from(elapsed) * granularity)
}

/// Returns the time since the kernel has started. On x86_64, extra precision
/// is provided by the APIC timer and has almost zero overhead (it's a simple
/// memory read), but on other architectures, it may be way more expensive.
#[must_use]
pub fn since_boot() -> Duration {
    let millis = get_jiffies() * (1000 / jiffies_frequency());
    let nanos = jiffies_nano_offset();
    Duration::from_millis(millis) + nanos
}
