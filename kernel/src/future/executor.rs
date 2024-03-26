use super::{
    task::{self, Task},
    waker::TaskWaker,
};
use alloc::collections::BTreeMap;
use config::MAX_TASKS;
use core::task::{Context, Poll, Waker};
use crossbeam::queue::ArrayQueue;

pub struct Executor {
    wakers: BTreeMap<task::Identifier, Waker>,
    tasks: BTreeMap<task::Identifier, Task>,
    queue: Arc<ArrayQueue<task::Identifier>>,
}

impl Executor {
    #[must_use]
    pub fn new() -> Self {
        Self {
            wakers: BTreeMap::new(),
            tasks: BTreeMap::new(),
            queue: Arc::new(ArrayQueue::new(MAX_TASKS as usize)),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let id = task.id();

        if let Some(task) = self.tasks.insert(id, task) {
            panic!("Task with id {:?} already exists", task.id());
        }
        self.queue.push(id).expect("Too many async tasks");
    }

    /// Polls the next task in the queue
    pub fn run_once(&mut self) {
        if let Some(id) = self.queue.pop() {
            // If the task is not in the executor, this means it has already
            // been completed and we can ignore it
            let task = match self.tasks.get_mut(&id) {
                Some(task) => task,
                None => return,
            };

            // Get the waker for the task or create a new one if it doesn't exist.
            // This avoid creating multiple wakers for a same task
            let waker = self
                .wakers
                .entry(task.id())
                .or_insert_with(|| TaskWaker::waker(Arc::clone(&self.queue), id))
                .clone();

            let context = &mut Context::from_waker(&waker);
            match task.poll(context) {
                Poll::Pending => {
                    // If the task is not ready, push it back to the queue
                    self.queue.push(id).expect("Too many async tasks");
                }
                Poll::Ready(()) => {
                    // Remove the task and waker from the executor
                    self.wakers.remove(&id);
                    self.tasks.remove(&id);
                }
            }
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
