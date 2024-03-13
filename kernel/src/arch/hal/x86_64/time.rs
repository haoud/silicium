use config::TIMER_HZ;
use time::unit::Millisecond;

use crate::arch::x86_64::apic;
use core::sync::atomic::Ordering;

/// Returns the number of jiffies since the kernel has started.
///
/// A jiffy is a unit of time used in the kernel. It is defined as
/// the number of ticks that the kernel has been running.
#[must_use]
pub fn get_jiffies() -> u64 {
    apic::local::timer::JIFFIES.load(Ordering::Relaxed)
}

/// Returns the granularity of the jiffies in nanoseconds, which is the
/// time between each jiffy.
#[must_use]
pub const fn jiffies_granularity() -> Millisecond {
    Millisecond::new(1_000 / TIMER_HZ as u64)
}
