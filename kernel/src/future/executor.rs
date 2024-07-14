use super::waker::NoopWaker;
use crate::arch;
use async_task::{Runnable, Task};
use core::task::{Context, Poll, Waker};
use crossbeam::queue::ArrayQueue;
use futures::Future;
use spin::lazy::Lazy;

/// The global executor instance that will run all the tasks in the system.
static EXECUTOR: Lazy<Executor> = Lazy::new(Executor::new);

/// The executor struct that will hold the tasks queue that are ready to be
/// polled.
pub struct Executor {
    queue: ArrayQueue<Runnable>,
}

impl Executor {
    /// Create a new executor with a fixed size empty queue (defined by the
    /// `MAX_TASKS` configuration)
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: ArrayQueue::new(config::MAX_TASKS as usize),
        }
    }

    /// Schedule a task to be run later when the executor will be ready
    /// to poll it.
    pub fn schedule_later(&self, runnable: Runnable) {
        self.queue.push(runnable).expect("Too many tasks")
    }

    /// Get the next task to run. This function will return `None` if there
    /// are no tasks to run.
    #[must_use]
    pub fn get_task(&self) -> Option<Runnable> {
        self.queue.pop()
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

/// Setup the executor. This function will force the executor to be initialized
/// and ready to run tasks.
#[inline]
pub fn setup() {
    Lazy::force(&EXECUTOR);
}

/// Spawn a future into the executor into a task and return the task handle and
/// the Task object. The `Runnable` object is used to schedule the task to be
/// run later, and the `Task` object is used to wait for the task to be
/// completed.
pub fn spawn<F>(future: F) -> (Runnable, Task<F::Output>)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    async_task::spawn(future, |runnable| {
        EXECUTOR.schedule_later(runnable);
    })
}

/// Spawn a future into the executor into a detached task. The future will be
/// executed in the background independently of the caller. This function is
/// useful when you want to run a task in the background without waiting for
/// its completion.
pub fn schedule_detached<F>(future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let (runnable, task) = spawn(future);
    runnable.schedule();
    task.detach();
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

/// Run the executor. This function will run indefinitely and will execute all
/// the tasks that are scheduled in the executor.
pub fn run() -> ! {
    // SAFETY: Enabling interrupts here is safe because the kernel should
    // have finished its initialization and should be able to receive any
    // interrupts that are triggered wihtout causing any issues.
    unsafe {
        arch::irq::enable();
    }

    loop {
        while let Some(task) = EXECUTOR.get_task() {
            task.run();
        }
        arch::irq::wait();
    }
}
