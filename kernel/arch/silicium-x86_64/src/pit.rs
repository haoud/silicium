pub use super::io::Port;

//static CHANNEL_0: Port<u8> = Port::new(0x40);
//static CHANNEL_1: Port<u8> = Port::new(0x41);
static CHANNEL_2: Port<u8> = Port::new(0x42);
static COMMAND: Port<u8> = Port::new(0x43);
static KBC_PORT_B: Port<u8> = Port::new(0x61);

/// The internal frequency of the PIT, in Hz. This is the frequency of the internal
/// clock that drives the PIT, and is not the frequency that the PIT can be set to.
pub const INTERNAL_FREQ: u64 = 1_193_180;

/// The number of nanoseconds between each PIT internal tick.
pub const PIT_TICK_NS: u64 = 1_000_000_000 / 1_193_180;

/// Configure the channel 2 of the PIT to reach 0 after at least `ms` milliseconds.
///
/// # Panics
/// Panic if `ms` is not in the range 1..100.
#[allow(clippy::cast_possible_truncation)]
pub fn prepare_sleep(ms: u64) {
    assert!(ms < 100, "ms must be less than 100");
    assert!(ms > 0, "ms must be greater than 0");

    // SAFETY: Clearing the speaker bit is safe and should not cause any
    // side effects that could lead to undefined behavior or memory unsafety.
    unsafe {
        KBC_PORT_B.clear_bits(0x01);
    }

    let initial = (ms * 1_000_000) / PIT_TICK_NS;
    let initial = initial as u16;

    // SAFETY: This is safe because we are configuring the channel 2 of the PIT,
    // which will not send IRQ when its internal counter reaches 0. Writing to
    // the I/O ports below is safe and should not cause any side effects that
    // could lead to undefined behavior or memory unsafety.
    unsafe {
        // Select the channel 2, one shot mode
        COMMAND.write(0xb2);
        CHANNEL_2.write((initial & 0xFF) as u8);
        CHANNEL_2.write((initial >> 8) as u8);

        // Start the timer
        KBC_PORT_B.set_bits(0x01);
    }
}

/// Perform a sleep after the PIT has been configured with `prepare_sleep`. This
/// function will block until the PIT internal counter reaches 0 (channel 2).
/// If the deadline has already passed, this function will return immediately. If
/// no call to `prepare_sleep` has been made, this function will return immediately
/// too.
pub fn perform_sleep() {
    // SAFETY: Polling the 5th bit of the KBC port B is safe and should not cause
    // any side effects that could lead to undefined behavior or memory unsafety.
    unsafe {
        KBC_PORT_B.poll_clear_bits(0x20);
    }
}
