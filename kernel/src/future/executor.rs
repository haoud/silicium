use super::{
    task::{self, Task},
    waker::{NoopWaker, TaskWaker},
};
use crate::{arch, library::spin::Spinlock};
use alloc::collections::BTreeMap;
use config::MAX_TASKS;
use core::task::{Context, Poll, Waker};
use crossbeam::queue::ArrayQueue;
use spin::lazy::Lazy;

/// The global executor that will run all async tasks
static EXECUTOR: Lazy<Executor> = Lazy::new(Executor::new);

/// A simple executor that runs async tasks. This executor is a simple FIFO
/// executor that runs tasks in the order they are spawned.
pub struct Executor {
    /// A map of wakers for each task. This avoid creating multiple wakers
    /// for a same task, which would be a waste of resources. This also allow
    /// to easily remove a waker when the task is completed.
    wakers: Spinlock<BTreeMap<task::Identifier, Waker>>,

    /// A map of tasks that are currently running in the executor
    tasks: Spinlock<BTreeMap<task::Identifier, Task>>,

    /// A queue of tasks that are ready to be polled
    queue: Arc<ArrayQueue<task::Identifier>>,
}

impl Executor {
    /// Creates a new executor
    #[must_use]
    pub fn new() -> Self {
        Self {
            wakers: Spinlock::new(BTreeMap::new()),
            tasks: Spinlock::new(BTreeMap::new()),
            queue: Arc::new(ArrayQueue::new(MAX_TASKS as usize)),
        }
    }

    /// Returns true if there is no task in the executor ready to be polled.
    /// The name of this function comes from the french word "chômage" that
    /// means "unemployment". I thought it was a funny name for this function
    /// (Yeah, that was more funny when it was 3AM...)
    #[must_use]
    pub fn chomage(&self) -> bool {
        self.queue.is_empty()
    }

    /// Spawns a new task in the executor. The task will be polled when the
    /// executor will be run with the `run_once` method.
    pub fn spawn(&self, task: Task) {
        let id = task.id();

        if let Some(task) = self.tasks.lock().insert(id, task) {
            panic!("Task with id {:?} already exists", task.id());
        }
        self.queue.push(id).expect("Too many async tasks");
    }

    /// Polls the next task in the queue. If the task is not ready, it will be
    /// pushed back to the end of the queue. If the task is ready, it will be
    /// removed from the executor as well as its waker.
    pub fn run_once(&self) {
        if let Some(id) = self.queue.pop() {
            // If the task is not in the executor, this means it has already
            // been completed and we can ignore it or that it is already
            // running and we can ignore it as well.
            let mut task = match self.tasks.lock().remove(&id) {
                Some(task) => task,
                None => return,
            };

            // Get the waker for the task or create a new one if it doesn't
            // exist. This avoid creating multiple wakers for a same task
            let waker = self
                .wakers
                .lock()
                .entry(task.id())
                .or_insert_with(|| {
                    TaskWaker::waker(Arc::clone(&self.queue), id)
                })
                .clone();

            let context = &mut Context::from_waker(&waker);
            match task.poll(context) {
                Poll::Pending => {
                    // Push the task back to the list of tasks
                    self.tasks.lock().insert(id, task);
                }
                Poll::Ready(()) => {
                    // Remove the waker from the executor
                    self.wakers.lock().remove(&id);
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
/// This function must be called only once during the startup of the kernel
/// and before any other async operation.
#[init]
pub unsafe fn setup() {
    Lazy::force(&EXECUTOR);
}

/// Spawn a new task in the executor. The task will be polled when the
/// executor will be run with the `run` function.
pub fn spawn(task: Task) {
    log::trace!("Spaning a new future (size: {})", task.future_size());
    EXECUTOR.spawn(task);
}

/// Block the current thread until the provided future is resolved. It will
/// simply poll the future until it is ready repeatedly, consuming CPU cycles.
///
/// This function is very useful for running async tasks in a synchronous
/// context, like during the kernel initialization.
pub fn block_on<F: core::future::Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWaker));
    let mut context = Context::from_waker(&waker);
    let mut pin = core::pin::pin!(future);
    loop {
        if let Poll::Ready(value) = pin.as_mut().poll(&mut context) {
            return value;
        }
    }
}

/// Run the executor. This function will run the executor in a loop, polling
/// tasks forever. Since this function will never return, it is marked as `!`.
///
/// # Safety
/// The caller must ensure that the stack in which the current thread is
/// running will remain valid for the entire duration of the kernel. The
/// caller must also ensure that enabling interrupts at this point will
/// not cause any issue.
pub unsafe fn run() -> ! {
    arch::irq::enable();
    loop {
        EXECUTOR.run_once();
        while EXECUTOR.chomage() {
            arch::irq::wait();
        }
    }
}
