use crate::time::{
    instant::Instant,
    timer::{self, Timer},
};
use core::{task::Waker, time::Duration};
use futures::Future;

/// A future that resolves after a specified duration of time has elapsed.
pub struct SleepFuture {
    /// The time when the sleep should expire.
    expire: Instant,

    /// The timer guard that is used to cancel the timer if the future is
    /// dropped or the sleep is completed without the timer being triggered.
    guard: Option<timer::Guard>,
}

impl SleepFuture {
    /// Creates a new `SleepFuture` that will resolve after the specified
    /// duration has elapsed since now.
    #[must_use]
    pub fn new(expire: Duration) -> Self {
        Self {
            expire: Instant::now() + expire,
            guard: None,
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
            if self.guard.is_none() {
                // Register the timer to wake up the task if it hasn't been
                // registered yet. The timer will wake up the task by calling
                // `wake_by_ref` on the waker.
                let guard = Timer::register(
                    self.expire,
                    Box::new(cx.waker().clone()),
                    |timer| {
                        timer
                            .data()
                            .downcast_mut::<Waker>()
                            .unwrap()
                            .wake_by_ref();
                    },
                );

                // If no guard was returned, the timer has already expired
                // and the callback has been executed. In this case, return
                // `Poll::Ready(())` immediately.
                if let Some(guard) = guard {
                    self.get_mut().guard = Some(guard);
                } else {
                    return core::task::Poll::Ready(());
                }
            }
            core::task::Poll::Pending
        } else {
            // Drop the guard to cancel the timer and return `Poll::Ready(())`
            core::mem::drop(self.get_mut().guard.take());
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
