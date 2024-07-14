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

/// Sleeps for at least the given duration. Due to the timer resolution, the
/// actual sleep time may be longer than the requested duration, but it will
/// never be shorter.
pub async fn sleep(duration: Duration) {
    SleepFuture::new(duration).await
}
