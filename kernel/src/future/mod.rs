pub mod executor;
pub mod mutex;
pub mod sleep;
pub mod timeout;
pub mod waker;

pub use mutex::Mutex;

/// Initialize the kernel async runtime
///
/// # Safety
/// This function must be called only once during the startup of the
/// kernel and before any other async operation.
#[init]
pub unsafe fn setup() {
    executor::setup();
}
