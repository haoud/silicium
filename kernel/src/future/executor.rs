use super::{
    task::{self, Task},
    waker::TaskWaker,
};
use crate::arch;
use alloc::collections::BTreeMap;
use config::MAX_TASKS;
use core::task::{Context, Poll, Waker};
use crossbeam::queue::ArrayQueue;
use spin::Spinlock;

/// The global executor that will run all async tasks
static EXECUTOR: Spinlock<Option<Executor>> = Spinlock::new(None);

/// A simple executor that runs async tasks. This executor is a simple FIFO executor
/// that runs tasks in the order they are spawned.
pub struct Executor {
    /// A map of wakers for each task. This avoid creating multiple wakers for a same
    /// task, which would be a waste of resources. This also allow to easily remove a
    /// waker when the task is completed.
    wakers: BTreeMap<task::Identifier, Waker>,

    /// A map of tasks that are currently running in the executor
    tasks: BTreeMap<task::Identifier, Task>,

    /// A queue of tasks that are ready to be polled
    queue: Arc<ArrayQueue<task::Identifier>>,
}

impl Executor {
    /// Creates a new executor
    #[must_use]
    pub fn new() -> Self {
        Self {
            wakers: BTreeMap::new(),
            tasks: BTreeMap::new(),
            queue: Arc::new(ArrayQueue::new(MAX_TASKS as usize)),
        }
    }

    /// Returns true if there is no task in the executor ready to be polled. The name of this
    /// function comes from the french word "chômage" that means "unemployment". I thought it
    /// was a funny name for this function (Yeah, that was more funny when it was 3AM...)
    #[must_use]
    pub fn chomage(&self) -> bool {
        self.queue.is_empty()
    }

    /// Spawns a new task in the executor. The task will be polled when the executor
    /// will be run with the `run_once` method.
    pub fn spawn(&mut self, task: Task) {
        let id = task.id();

        if let Some(task) = self.tasks.insert(id, task) {
            panic!("Task with id {:?} already exists", task.id());
        }
        self.queue.push(id).expect("Too many async tasks");
    }

    /// Polls the next task in the queue. If the task is not ready, it will be pushed
    /// back to the end of the queue. If the task is ready, it will be removed from
    /// the executor as well as its waker.
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

/// Initialize the executor.
///
/// # Safety
/// This function must be called only once during the startup of the kernel and before
/// any other async operation.
#[init]
pub unsafe fn setup() {
    EXECUTOR.lock().replace(Executor::new());
}

/// Spawn a new task in the executor. The task will be polled when the executor will be
/// run with the `run` function.
pub fn spawn(task: Task) {
    EXECUTOR
        .lock()
        .as_mut()
        .expect("Executor not initialized")
        .spawn(task);
}

/// Run the executor. This function will run the executor in a loop, polling tasks
/// forever. Since this function will never return, it is marked as `!`.
///
/// # Safety
/// The caller must ensure that the stack in which the current thread is running will
/// remain valid for the entire duration of the kernel.
pub unsafe fn run() -> ! {
    // Wait until the executor is initialized
    while EXECUTOR.lock().as_ref().is_none() {
        arch::irq::wait();
    }

    loop {
        EXECUTOR.lock().as_mut().unwrap().run_once();
        // TODO: Maybe we can wait until a task is ready to be polled instead of
        // waiting for an interruption to occur ? Using an async task maybe...
        while EXECUTOR.lock().as_mut().unwrap().chomage() {
            arch::irq::wait();
        }
    }
}
