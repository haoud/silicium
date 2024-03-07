struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, _record: &log::Record) {}
    fn flush(&self) {}
}

/// Setup the architecture dependent logger.
#[inline]
pub fn setup() {}

/// Force the logger to be unlocked. This is useful when a panic
/// occurs and the serial port is locked, to avoid a deadlock.
///
/// # Safety
/// This function is unsafe because it can cause a data race if the logger
/// was still being used when this function is called. It should be called
/// only when a panic occurs.
pub unsafe fn unlock() {}
