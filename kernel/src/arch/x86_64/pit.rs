pub use crate::arch::x86_64::io::Port;

static CHANNEL_0: Port<u8> = Port::new(0x40);
static CHANNEL_1: Port<u8> = Port::new(0x41);
static CHANNEL_2: Port<u8> = Port::new(0x42);
static COMMAND: Port<u8> = Port::new(0x43);
static KBC_PORT_B: Port<u8> = Port::new(0x61);

/// The different channels available on the PIT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Channel {
    Channel0,
    Channel1,
    Channel2,
}

/// The internal frequency of the PIT, in Hz. This is the frequency of the
/// internal clock that drives the PIT, and is not the frequency that the
/// PIT can be set to.
pub const INTERNAL_FREQ: u64 = 1_193_180;

/// The number of nanoseconds between each PIT internal tick.
pub const PIT_TICK_NS: u64 = 1_000_000_000 / 1_193_180;

/// Configure the channel 2 of the PIT to reach 0 after at least `ms`
/// milliseconds.
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
        KBC_PORT_B.clear_bits(0x03);
    }

    let initial = (ms * 1_000_000) / PIT_TICK_NS;
    let initial = initial as u16;

    // SAFETY: This is safe because we are configuring the channel 2 of the
    // PIT, which will not send IRQ when its internal counter reaches 0.
    // Writing to the I/O ports below is safe and should not cause any side
    // effects that could lead to undefined behavior or memory unsafety.
    unsafe {
        // Select the channel 2, one shot mode
        COMMAND.write(0xb2);
        CHANNEL_2.write((initial & 0xFF) as u8);
        CHANNEL_2.write((initial >> 8) as u8);
    }
}

/// Perform a sleep after the PIT has been configured with `prepare_sleep`.
/// This function will block until the PIT internal counter reaches 0 on the
/// channel 2. If no call to `prepare_sleep` has been made, this function
/// will return immediately too.
pub fn perform_sleep() {
    while read_counter(Channel::Channel2) > 0 {}
}

/// Read the current value of the counter of the PIT for the given channel.
/// This function is slow because it perform one I/O read and two I/O writes,
/// and on x86_64, I/O operations are very slow...
pub fn read_counter(channel: Channel) -> u16 {
    let port = match channel {
        Channel::Channel0 => &CHANNEL_0,
        Channel::Channel1 => &CHANNEL_1,
        Channel::Channel2 => &CHANNEL_2,
    };

    // SAFETY: Reading the PIT counter is safe and should not lead to UB
    // nor memory unsafety.
    unsafe {
        COMMAND.write(0xe0 | ((channel as u8) << 6));
        let lo = port.read();
        let hi = port.read();
        (hi as u16) << 8 | lo as u16
    }
}
