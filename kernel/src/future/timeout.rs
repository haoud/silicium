use crate::time::{instant::Instant, timer::Timer};
use core::{pin::Pin, task::Poll, time::Duration};
use futures::Future;

/// A future that ensures that a future completes before a specified
/// deadline. If the future completes before the deadline, the output
/// is returned, otherwise `None` is returned.
///
/// # Warning
/// If the future blocks without yielding, the timeout will not be
/// able to wake up the task when the deadline is reached because
/// futures are scheduled cooperatively. This means that the future
/// must yield to the executor in order for the timeout to work, and
/// that why futures should not block and are not appropriate for
/// CPU-bound tasks.
#[derive(Debug)]
pub struct Timeout<T> {
    /// The deadline before which the future must complete.
    deadline: Instant,

    /// The future that is being timed out.
    future: Pin<Box<T>>,
}

impl<T: Future> Future for Timeout<T> {
    /// The output of the future is an `Option` that is `Some` if the
    /// future completes before the deadline, otherwise `None`.
    type Output = Option<T::Output>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context,
    ) -> core::task::Poll<Self::Output> {
        let this = self.get_mut();
        if Instant::now() >= this.deadline {
            Poll::Ready(None)
        } else {
            // Add a timer to the future that wakes up the task when the
            // deadline is reached and then poll the future.
            Timer::register(this.deadline, Box::new(cx.waker().clone()));
            match this.future.as_mut().poll(cx) {
                Poll::Ready(output) => Poll::Ready(Some(output)),
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

/// Requires that the future completes before the specified duration.
///
/// If the future completes before the duration has elapsed, then the
/// completed value is returned. Otherwise, an error is returned and
/// the future is canceled.
///
/// Note that the timeout is checked before polling the future, so if
/// the future does not yield during execution then it is possible for
/// the future to complete and exceed the timeout _without_ returning
/// an error.
///
/// Futhermore, the future can expire without being polled once if
/// the executor is busy and does not poll the future before the
/// deadline is reached.
pub fn timeout<T>(duration: Duration, future: T) -> Timeout<T>
where
    T: Future,
{
    Timeout {
        future: Box::pin(future),
        deadline: Instant::now() + duration,
    }
}
