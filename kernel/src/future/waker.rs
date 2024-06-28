use super::task;
use alloc::task::Wake;
use crossbeam::queue::ArrayQueue;

/// A waker for a task to allow it to be woken up and be polled again.
#[derive(Debug)]
pub struct TaskWaker {
    /// The queue to push the task identifier to when the task is woken up.
    queue: Arc<ArrayQueue<task::Identifier>>,

    /// The identifier of the task to wake up.
    id: task::Identifier,
}

impl TaskWaker {
    /// Create a new task waker for the given task identifier. The task waker
    /// will push the task identifier to the given queue when the task will be
    /// woken up.
    #[must_use]
    pub fn new(
        queue: Arc<ArrayQueue<task::Identifier>>,
        id: task::Identifier,
    ) -> Self {
        Self { queue, id }
    }

    /// Create a new waker for the given task identifier. The task waker will
    /// push the task identifier to the given queue when the task will be woken
    /// up.
    #[must_use]
    pub fn waker(
        queue: Arc<ArrayQueue<task::Identifier>>,
        id: task::Identifier,
    ) -> core::task::Waker {
        core::task::Waker::from(Arc::new(Self::new(queue, id)))
    }

    /// Get the identifier of the task to wake up.
    #[must_use]
    pub const fn task_id(&self) -> task::Identifier {
        self.id
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.queue.push(self.id).expect("Too many async tasks");
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.queue.push(self.id).expect("Too many async tasks");
    }
}
