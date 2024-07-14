use crate::time::{instant::Instant, timer::Timer};
use core::time::Duration;
use futures::Future;

/// A future that resolves after a specified duration of time has elapsed.
pub struct SleepFuture {
    /// The time when the sleep should expire.
    expire: Instant,
}

impl SleepFuture {
    /// Creates a new `SleepFuture` that will resolve after the specified
    /// duration has elapsed since now.
    #[must_use]
    pub fn new(expire: Duration) -> Self {
        Self {
            expire: Instant::now() + expire,
        }
    }
}

impl Future for SleepFuture {
    type Output = ();

    /// Polls the `SleepFuture`. If the sleep has not yet expired, the waker is
    /// registered with the timer and the future returns `Pending`. If the sleep
    /// has expired, the future returns `Ready`.
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context,
    ) -> core::task::Poll<Self::Output> {
        if self.expire > Instant::now() {
            Timer::register(self.expire, Box::new(cx.waker().clone()));
            core::task::Poll::Pending
        } else {
            core::task::Poll::Ready(())
        }
    }
}

/// A future that yield the current task and put it at the end of the task
/// queue. The first pool of this future will always return `Poll::Pending`,
/// to ensure that the task is put at the end of the task queue.
#[derive(Debug)]
pub struct YieldFuture {
    polled: bool,
}

impl YieldFuture {
    /// Create a new yield future
    #[must_use]
    pub const fn new() -> Self {
        Self { polled: false }
    }
}

impl Default for YieldFuture {
    fn default() -> Self {
        Self::new()
    }
}

impl Future for YieldFuture {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        if self.polled {
            core::task::Poll::Ready(())
        } else {
            self.get_mut().polled = true;
            cx.waker().wake_by_ref();
            core::task::Poll::Pending
        }
    }
}

/// Yield the current task and put it at the end of the task queue
pub async fn yield_now() {
    YieldFuture::new().await
}

/// Sleeps for at least the given duration. Due to the timer resolution, the
/// actual sleep time may be longer than the requested duration, but it will
/// never be shorter.
pub async fn sleep(duration: Duration) {
    SleepFuture::new(duration).await
}
