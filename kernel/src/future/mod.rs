pub mod executor;
pub mod sleep;
pub mod task;
pub mod waker;

pub use executor::Executor;
pub use task::Task;
pub use waker::TaskWaker;

/// Initialize the kernel async runtime
///
/// # Safety
/// This function must be called only once during the startup of the
/// kernel and before any other async operation.
#[init]
pub unsafe fn setup() {}
