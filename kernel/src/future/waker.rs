/// A waker that does nothing when woken up. It is used when a task is polled
/// repeatedly and does not need to be woken up explicitly.
pub struct NoopWaker;

impl alloc::task::Wake for NoopWaker {
    fn wake(self: Arc<Self>) {
        // Do nothing
    }

    fn wake_by_ref(self: &Arc<Self>) {
        // Do nothing
    }
}
