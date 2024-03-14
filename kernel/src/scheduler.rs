use crate::arch::{
    self,
    thread::{State, Thread},
};
use alloc::collections::VecDeque;
use core::num::Saturating;
use macros::per_cpu;
use spin::Spinlock;

/// The current task running on the CPU. If this is `None`, then the CPU is idle.
#[per_cpu]
pub static CURRENT: Spinlock<Option<Task>> = Spinlock::new(None);

/// The scheduler for the kernel
pub static SCHEDULER: Scheduler = Scheduler::new();

/// The default time slice for a task in ticks
pub const DEFAULT_QUANTUM: u64 = 50;

/// A simple round-robin scheduler for the kernel. It is clear that this
/// is suboptimal, but it is simple and it works enough for now.
#[derive(Debug)]
pub struct Scheduler {
    /// The tasks that are ready to run
    ready: Spinlock<VecDeque<Task>>,
}

impl Scheduler {
    /// Create a new scheduler
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ready: Spinlock::new(VecDeque::new()),
        }
    }

    /// Set the state of the current task to `Running` and enqueue it to the
    /// ready queue. The task will be scheduled to run when all tasks enqueued
    /// before it have been scheduled.
    pub fn enqueue_ready(&self, task: Task) {
        task.thread.lock().set_state(State::Ready);
        self.ready.lock().push_back(task);
    }

    /// Pop the next task that is ready to run and set its state to `Running`.
    /// If there are no tasks ready to run, then this function will return `None`.
    #[must_use]
    pub fn pop(&self) -> Option<Task> {
        self.ready.lock().pop_front().inspect(|task| {
            task.thread.lock().set_state(State::Running);
        })
    }

    /// Pop the next task that is ready to run and set its state to `Running`.
    /// If there are no tasks ready to run, then this function will idle the CPU
    /// until a task is ready to run.
    pub fn pop_or_idle(&self) -> Task {
        arch::irq::without(|| {
            loop {
                if let Some(next) = self.pop() {
                    break next;
                }

                // We have no tasks to run, so we will just idle
                // TODO: Use arch::preempt_point()
                unsafe {
                    arch::irq::enable();
                    arch::irq::wait();
                    arch::irq::disable();
                }
            }
        })
    }

    /// Schedule the current running task to the next task. This function will
    /// save the current task's registers and then jump to the next task.
    ///
    /// When the current thread will be resumed, it will return to the [`schedule_to`]
    /// function normally, as if it was never interrupted.
    pub fn schedule_to(&self, next: Task) {
        arch::irq::without(|| {
            let current = CURRENT
                .local()
                .lock()
                .take()
                .expect("Cannot schedule without a task");

            let state = current.thread.lock().state();
            let switch =
                arch::thread::prepare_switch(&mut current.thread.lock(), &mut next.thread.lock());

            match state {
                State::Terminated(_) | State::Exited(_) => {
                    // TODO: Maybe jump to the next task directly instead of
                    // switching ? This should be a bit faster
                }
                State::Sleeping | State::Waiting => {
                    // Nothing to do... This is not yet implemented
                }
                State::Running => {
                    // The task was running, so we should enqueue it back to the
                    // ready queue to be scheduled again later
                    self.enqueue_ready(current);
                }
                _ => {
                    unreachable!("Invalid state for scheduling a task: {:?}", state);
                }
            }

            set_current_task(next);

            // SAFETY: This should be safe since we can assume that the registers
            // saved previously are valid, and we does not have any locks held
            // (that could lead to a deadlock when switching tasks)
            unsafe {
                arch::thread::perform_switch(switch);
            }
        });
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// A task that can be scheduled to run on the CPU
#[derive(Debug)]
pub struct Task {
    /// The thread that the task is running
    thread: Arc<Spinlock<Thread>>,

    /// The time slice remaining for the task. If this is `0`, then the task
    /// will be preempted if another task is ready to run.
    remaning: Saturating<u64>,
}

impl Task {
    /// Create a new task with the given thread and with the default
    /// time slice
    #[must_use]
    pub fn new(thread: Thread) -> Self {
        Self {
            thread: Arc::new(Spinlock::new(thread)),
            remaning: Saturating(DEFAULT_QUANTUM),
        }
    }

    /// Restore the time slice for the task
    pub fn restore_quantum(&mut self) {
        self.remaning = Saturating(DEFAULT_QUANTUM);
    }
}

/// Decrease the time slice for the current task. If the time slice is `0`, then
/// the task will be preempted if another task is ready to run. If there are no
/// tasks ready to run, then the current task will continue to run until another
/// task is ready to run.
pub fn tick() {
    let current = CURRENT.local();
    let mut current = current.lock();

    // If there is no task running, then we have nothing to do
    if let Some(task) = current.as_mut() {
        // Decrease the time slice for the task and check if it is time to
        // preempt the task
        task.remaning -= 1;
        if task.remaning.0 == 0 {
            // If there is no other task ready to run, we can simply delay the
            // scheduling of the current task because there is nothing better to
            // do. The task will be scheduled when another task is will be ready
            // to run.
            if let Some(next) = SCHEDULER.pop() {
                // Restore the time slice to allow the current task to run again
                // when it will be resumed by the scheduler
                task.restore_quantum();
                core::mem::drop(current);
                SCHEDULER.schedule_to(next);
            }
        }
    }
}

/// Enter into the scheduler. This function will idle the CPU until a task is
/// ready to run, and then it will jump to that task without returning.
///
/// This function should only be used after the kernel has been initialized, to
/// start the first task on the core.
pub fn enter() -> ! {
    let task = SCHEDULER.pop_or_idle();
    let jump = arch::thread::prepare_jump(&mut task.thread.lock());

    set_current_task(task);

    // SAFETY: We assume that the task is valid, and we does not have any
    // locks held (that could lead to a deadlock when switching tasks)
    unsafe {
        arch::thread::perform_jump(jump);
    }
}

/// Set the current task for the CPU and return the previous task
pub fn set_current_task(task: Task) -> Option<Task> {
    CURRENT.local().lock().replace(task)
}
