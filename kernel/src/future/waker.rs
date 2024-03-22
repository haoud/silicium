use super::task;
use alloc::task::Wake;
use crossbeam::queue::ArrayQueue;

pub struct TaskWaker {
    queue: Arc<ArrayQueue<task::Identifier>>,
    id: task::Identifier,
}

impl TaskWaker {
    #[must_use]
    pub fn new(queue: Arc<ArrayQueue<task::Identifier>>, id: task::Identifier) -> Self {
        Self { queue, id }
    }

    #[must_use]
    pub fn waker(
        queue: Arc<ArrayQueue<task::Identifier>>,
        id: task::Identifier,
    ) -> core::task::Waker {
        let waker = Self::new(queue, id);
        core::task::Waker::from(Arc::new(waker))
    }

    #[must_use]
    pub const fn id(&self) -> task::Identifier {
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
