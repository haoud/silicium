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

/// The shift value of a page. The value is always 12.
pub const PAGE_SHIFT: u8 = 12;
const_assert!(PAGE_SHIFT == 12);
const_assert!((1 << PAGE_SHIFT) == PAGE_SIZE);

/// The maximum number of kernel handle that can be allocated across all processes.
/// Diminishing this value can reduce the memory consumption of the kernel, but it
/// will also limit the number of handles that can be created. A too low value can
/// prevent the system from working properly.
pub const MAX_HANDLES: u32 = 1024;
const_assert!(MAX_HANDLES >= 1024);

/// The maximum numbers of asynchronous tasks that can be created. Since each thread
/// is attached to an kernel stack, this value should be AT LEAST twice the number of
/// threads that can be created (and I'm pretty sure it should be greater than that).
pub const MAX_TASKS: u32 = 1024;
const_assert!(MAX_TASKS >= 8);

/// The size of the kernel stack in bytes. This value must be a multiple of the page
/// size and must be greater or equal to 8192 bytes.
pub const KSTACK_SIZE: usize = 8192;
const_assert!(KSTACK_SIZE >= 8192);
