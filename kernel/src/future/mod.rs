use core::time::Duration;

pub mod executor;
pub mod sleep;
pub mod task;
pub mod timeout;
pub mod waker;

pub use executor::{block_on, Executor};
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
    executor::spawn(Task::new(timeout_test()));
}

async fn tic() {
    loop {
        log::info!("Tic");
        sleep::sleep(Duration::from_secs(2)).await;
    }
}

async fn tac() {
    loop {
        sleep::sleep(Duration::from_secs(1)).await;
        log::info!("Tac");
        sleep::sleep(Duration::from_secs(1)).await;
    }
}

async fn timeout_test() {
    let duration = Duration::from_secs(2);
    let timeout = Duration::from_millis(1500);

    match timeout::timeout(timeout, sleep::sleep(duration)).await {
        Some(_) => log::info!("Timeout did not occur !"),
        None => log::info!("Timeout occurred !"),
    }
}
