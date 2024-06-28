use time::unit::Nanosecond;

pub mod executor;
pub mod sleep;
pub mod task;
pub mod thread;
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
pub unsafe fn setup() {
    executor::setup();
    executor::spawn(Task::new(tic()));
    executor::spawn(Task::new(tac()));
}

pub async fn tic() {
    loop {
        log::info!("Tic");
        sleep::sleep(Nanosecond::new(2_000_000)).await;
    }
}

pub async fn tac() {
    loop {
        sleep::sleep(Nanosecond::new(1_000_000)).await;
        log::info!("Tac");
        sleep::sleep(Nanosecond::new(1_000_000)).await;
    }
}
