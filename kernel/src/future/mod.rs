pub mod executor;
pub mod sleep;
pub mod task;
pub mod waker;

use core::time::Duration;

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
        sleep::sleep(Duration::from_secs(2)).await;
    }
}

pub async fn tac() {
    loop {
        sleep::sleep(Duration::from_secs(1)).await;
        log::info!("Tac");
        sleep::sleep(Duration::from_secs(1)).await;
    }
}
