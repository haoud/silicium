use crate::arch::x86_64::serial::{Port, Serial};
use core::fmt::Write;

/// The logger for the `x86_64` architecture. This logger writes messages to the
/// serial port COM1. If the serial port is not available, the logger does nothing.
static LOGGER: Logger = Logger::uninitialized();

/// The logger for the `x86_64` architecture. It's a simple logger that encapsulates
/// a serial port (COM1) and writes messages to it.
struct Logger {
    serial: spin::Mutex<Option<Serial>>,
}

impl Logger {
    /// Create a new uninitialized logger.
    #[must_use]
    pub const fn uninitialized() -> Self {
        Self {
            serial: spin::Mutex::new(None),
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            if let Some(serial) = self.serial.lock().as_mut() {
                let level = match record.level() {
                    log::Level::Error => "\x1B[1m\x1b[31m[!]\x1b[0m",
                    log::Level::Warn => "\x1B[1m\x1b[33m[-]\x1b[0m",
                    log::Level::Info => "\x1B[1m\x1b[32m[*]\x1b[0m",
                    log::Level::Debug => "\x1B[1m\x1b[34m[#]\x1b[0m",
                    log::Level::Trace => "\x1B[1m\x1b[35m[~]\x1b[0m",
                };
                _ = writeln!(serial, "{} {}", level, record.args());
            }
        }
    }

    fn flush(&self) {}
}

/// Setup the architecture dependent logger.
#[inline]
pub fn setup() {
    *LOGGER.serial.lock() = Serial::new(Port::COM1);
    log::set_max_level(log::LevelFilter::Trace);
    _ = log::set_logger(&LOGGER);
}

/// Write a message to the serial port. If the serial port is not initialized or
/// not available, this function does nothing. If an error occurs while writing
/// the message, the error is ignored.
pub fn write(message: &str) {
    if let Some(serial) = LOGGER.serial.lock().as_ref() {
        for character in message.bytes() {
            _ = serial.send(character);
        }
    }
}

/// Force the logger to unlock the serial port. This is useful when a panic
/// occurs and the serial port is locked, to avoid a deadlock.
///
/// # Safety
/// This function is unsafe because it can cause a data race if the logger
/// was still being used when this function is called. It should be called
/// only when a panic occurs.
pub unsafe fn unlock() {
    LOGGER.serial.force_unlock();
}
