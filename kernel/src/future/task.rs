use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};

/// A task that can be polled to completion
pub struct Task {
    /// The future that the task is running
    future: Pin<Box<dyn Future<Output = ()> + Send>>,

    /// The priority of the task. This is used to determine the order in which
    /// the tasks are executed.
    priority: Priority,

    /// The unique identifier for the task. This is used to identify the task
    /// in the task queue.
    id: Identifier,
}

impl Task {
    /// Create a new task from a future
    #[must_use]
    pub fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
        Self {
            future: Box::pin(future),
            id: Identifier::generate(),
            priority: Priority::default(),
        }
    }

    /// Poll the task
    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }

    /// Set the priority of the task
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    /// Get the priority of the task
    #[must_use]
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Get the unique identifier for the task
    #[must_use]
    pub const fn id(&self) -> Identifier {
        self.id
    }
}

/// A unique identifier for a task
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(u64);

impl Identifier {
    /// Generate a new unique identifier for a task
    #[must_use]
    pub fn generate() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl From<Identifier> for u64 {
    fn from(identifier: Identifier) -> u64 {
        identifier.0
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    /// The idle priority is the lowest priority. It is used for tasks that
    /// are not critical and can be executed when no other tasks are running.
    /// In the worst case, the idle tasks will be never executed.
    Idle,

    /// The low priority is used for tasks that are not critical but should
    /// still be executed in a timely manner. Background tasks can use this
    /// priority.
    Deamon,

    /// The normal priority is the default priority. It should be used for the
    /// majority of the tasks.
    #[default]
    Normal,

    /// The high priority is used for tasks that are critical and should be
    /// executed as soon as possible, but not in a real-time manner. For example,
    /// a video game because a missed deadline will not cause a critical failure.
    High,

    /// The realtime priority is the highest priority. It should be used for
    /// tasks that are critical and must be executed in a real-time manner. Silicium
    /// does not support hard real-time, but only a poor soft real-time. Silicium
    /// will probably never be used in a place where a missed deadline will cause
    /// a critical failure, so it's fine.
    Realtime,
}

/// A future that yield the current task and put it at the end of the task queue.
/// The first pool of this future will always return `Poll::Pending`, to ensure
/// that the task is put at the end of the task queue.
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

impl Future for YieldFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(())
        } else {
            self.get_mut().polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Yield the current task and put it at the end of the task queue
pub async fn yield_now() {
    YieldFuture::new().await
}
