use core::time::Duration;

pub mod executor;
pub mod mutex;
pub mod sleep;
pub mod timeout;
pub mod waker;

/// Initialize the kernel async runtime
///
/// # Safety
/// This function must be called only once during the startup of the
/// kernel and before any other async operation.
#[init]
pub unsafe fn setup() {
    executor::setup();
    executor::schedule_detached(tic());
    executor::schedule_detached(tac());
    executor::schedule_detached(timeout_test());
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
