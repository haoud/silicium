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
        }
    }

    /// Poll the task to completion
    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
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
