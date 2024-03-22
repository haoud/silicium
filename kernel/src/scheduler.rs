use crate::{
    arch,
    sys::{
        process::{self, Process},
        thread::{State, Thread},
    },
};
use alloc::collections::VecDeque;
use core::ops::{AddAssign, SubAssign};
use spin::Spinlock;

/// The current task running on the CPU. If this is `None`, then the CPU is idle.
#[per_cpu]
pub static CURRENT: Spinlock<Option<Thread>> = Spinlock::new(None);

/// The scheduler for the kernel
pub static SCHEDULER: Scheduler = Scheduler::new();

/// The default time slice for a task in ticks
pub const DEFAULT_QUANTUM: u64 = 50;

/// A simple round-robin scheduler for the kernel. It is clear that this
/// is suboptimal, but it is simple and it works enough for now.
#[derive(Debug)]
pub struct Scheduler {
    /// The tasks that are ready to run
    ready: Spinlock<VecDeque<Thread>>,
}

impl Scheduler {
    /// Create a new scheduler
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ready: Spinlock::new(VecDeque::new()),
        }
    }

    /// Set the state of the task to `Ready` and enqueue it to the ready queue. The task
    /// will be scheduled to run when all tasks enqueued before it have been scheduled.
    pub fn enqueue_ready(&self, mut task: Thread) {
        task.set_state(State::Ready);
        self.ready.lock().push_back(task);
    }

    /// Pop the next task that is ready to run and set its state to `Running`.
    /// If there are no tasks ready to run, then this function will return `None`.
    #[must_use]
    pub fn pop(&self) -> Option<Thread> {
        self.ready.lock().pop_front().map(|mut thread| {
            thread.set_state(State::Running);
            thread
        })
    }

    /// Pop the next task that is ready to run and set its state to `Running`.
    /// If there are no tasks ready to run, then this function will idle the CPU
    /// until a task is ready to run.
    pub fn pop_or_idle(&self) -> Thread {
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
    pub fn schedule_to(&self, mut next: Thread) {
        arch::irq::without(|| {
            let mut current = CURRENT
                .local()
                .lock()
                .take()
                .expect("Cannot schedule without a task");

            let state = current.state();
            let switch = arch::context::prepare_switch(current.context_mut(), next.context_mut());

            match state {
                State::Terminated(_) | State::Exited(_) => {
                    // TODO: Maybe jump to the next task directly instead of
                    // switching ? This should be a bit faster
                }
                State::Sleeping | State::Waiting => {
                    // Nothing to do... This is not yet implemented
                    // TODO: Put the task inside a queue to be woken up later
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

            CURRENT.local().lock().replace(next);

            // SAFETY: This should be safe since we can assume that the registers
            // saved previously are valid, and we does not have any locks held
            // (that could lead to a deadlock when switching tasks)
            unsafe {
                arch::context::perform_switch(switch);
            }
        });
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Setup the scheduler for the kernel. This function will create the kernel
/// process, which will be the parent of all kernel threads.
///
/// # Safety
/// This function is marked as unsafe because it must be called only once at the start
/// of the kernel. Failing to do so will result in undefined behavior.
#[init]
pub unsafe fn setup() {
    process::register(Process::new());
}

/// Decrease the time slice for the current task. If the time slice is `0`, then
/// the task will be preempted if another task is ready to run. If there are no
/// tasks ready to run, then the current task will continue to run until another
/// task is ready to run.
pub fn tick() {
    let current = CURRENT.local();
    let mut current = current.lock();

    // If there is no task running, then we have nothing to do
    if let Some(thread) = current.as_mut() {
        thread.quantum_mut().sub_assign(1);
        if thread.quantum().0 == 0 {
            // If there is no other task ready to run, we can simply delay the
            // scheduling of the current task because there is nothing better to
            // do. The task will be scheduled when another task is will be ready
            // to run.
            if let Some(next) = SCHEDULER.pop() {
                thread.quantum_mut().add_assign(DEFAULT_QUANTUM);
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
    let mut thread = SCHEDULER.pop_or_idle();
    let jump = arch::context::prepare_jump(thread.context_mut());

    // SAFETY: Loading the page table of the task should be safe since we can
    // only assume that it was correctly initialized.
    unsafe {
        thread.process().page_table().lock().load_current();
    }

    CURRENT.local().lock().replace(thread);

    // SAFETY: We assume that the task is valid, and we does not have any
    // locks held (that could lead to a deadlock when switching tasks)
    unsafe {
        arch::context::perform_jump(jump);
    }
}
