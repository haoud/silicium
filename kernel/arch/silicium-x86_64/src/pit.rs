pub use super::io::Port;

static CHANNEL_0: Port<u8> = Port::new(0x40);
static CHANNEL_1: Port<u8> = Port::new(0x41);
static CHANNEL_2: Port<u8> = Port::new(0x42);
static COMMAND: Port<u8> = Port::new(0x43);

static KBC_PORT_B: Port<u8> = Port::new(0x61);

/// The internal frequency of the PIT, in Hz. This is the frequency of the internal clock
/// that drives the PIT, and is not the frequency that the PIT can be set to.
pub const INTERNAL_FREQ: u64 = 1_193_180;

/// The number of nanoseconds between each PIT internal tick.
pub const PIT_TICK_NS: u64 = 1_000_000_000 / 1_193_180;

pub fn prepare_sleep(ms: u64) {
    assert!(ms > 0, "ms must be greater than 0");
    assert!(ms < 1000, "ms must be less than 1000");

    unsafe {
        KBC_PORT_B.write(KBC_PORT_B.read() & 0xFC);
    }

    let initial = (ms * 1_000_000) / PIT_TICK_NS;
    let initial = initial as u16;

    unsafe {
        // Select the channel 2, one shot mode
        COMMAND.write(0xb2);
        CHANNEL_2.write((initial & 0xFF) as u8);
        CHANNEL_2.write((initial >> 8) as u8);
        KBC_PORT_B.write(KBC_PORT_B.read() | 0x01);
    }
}

pub fn perform_sleep() {
    unsafe {
        while (KBC_PORT_B.read() & 0x20) != 0 {
            core::hint::spin_loop();
        }
    }
}
