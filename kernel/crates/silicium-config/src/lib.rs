#![no_std]
use static_assertions::const_assert;

/// The frequency of the timer interrupt in Hz. Must be 10, 100, 250, or 1000.
/// Lower values are most efficient but will result to an less responsive system,
/// while higher values will result to a more responsive system but will consume
/// more CPU time.
///
/// # Recommended values
/// - 10 Hz: For servers and other systems that don't need to be very responsive.
/// - 100 Hz: For low-end desktops and laptops.
/// - 250 Hz: The recommended value for most systems.
/// - 1000 Hz: For high-end desktops and laptops or for a more responsive system.
pub const TIMER_HZ: u16 = 1000;
const_assert!(TIMER_HZ == 10 || TIMER_HZ == 100 || TIMER_HZ == 250 || TIMER_HZ == 1000);

/// The size of a page in bytes. The value is always 4096.
pub const PAGE_SIZE: u16 = 4096;
const_assert!(PAGE_SIZE == 4096);
