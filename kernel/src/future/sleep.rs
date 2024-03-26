use crate::{
    arch,
    time::timer::{self, Timer},
};
use core::task::Waker;
use futures::Future;
use time::{unit::Nanosecond, Timespec};

pub struct SleepFuture {
    expire: Timespec,
    guard: Option<timer::Guard>,
}

impl SleepFuture {
    pub fn new(expire: Timespec) -> Self {
        Self {
            expire,
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
        if arch::time::current_timespec() < self.expire {
            if self.guard.is_none() {
                // Register the timer to wake up the task if it hasn't been registered yet.
                // The timer will wake up the task by calling `wake_by_ref` on the waker.
                self.get_mut().guard = Some(Timer::register(
                    self.expire,
                    Box::new(cx.waker().clone()),
                    |timer| {
                        timer
                            .data()
                            .downcast_mut::<Waker>()
                            .expect("Invalid downcast to core::future::Waker")
                            .wake_by_ref();
                    },
                ));
            }
            core::task::Poll::Pending
        } else {
            // Drop the guard to cancel the timer and return `Poll::Ready(())`
            core::mem::drop(self.get_mut().guard.take());
            core::task::Poll::Ready(())
        }
    }
}

/// Sleeps for at least the given duration. Due to the timer resolution, the actual
/// sleep time may be longer than the requested duration, but it will never be shorter.
pub async fn sleep(duration: impl Into<Nanosecond>) {
    SleepFuture::new(arch::time::current_timespec() + duration.into()).await
}
