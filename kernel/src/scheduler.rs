use crate::arch::{self, thread::Thread};
use alloc::{collections::VecDeque, sync::Arc};
use core::num::Wrapping;
use macros::per_cpu;
use spin::Spinlock;

/// The default time slice for a task in ticks
pub const DEFAULT_QUANTUM: u64 = 50;

/// The current task running on the CPU. If this is `None`, then the CPU is idle.
#[per_cpu]
pub static CURRENT: Spinlock<Option<Task>> = Spinlock::new(None);

/// The scheduler for the kernel
pub static SCHEDULER: Scheduler = Scheduler::new();

#[derive(Debug)]
pub struct Scheduler {
    /// The tasks that are ready to run
    ready: Spinlock<VecDeque<Task>>,
}

impl Scheduler {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ready: Spinlock::new(VecDeque::new()),
        }
    }

    pub fn enqueue_ready(&self, task: Task) {
        self.ready.lock().push_back(task);
    }

    #[must_use]
    pub fn pop(&self) -> Option<Task> {
        self.ready.lock().pop_front()
    }

    pub fn pop_or_idle(&self) -> Task {
        loop {
            if let Some(next) = self.pop() {
                break next;
            }

            // We have no tasks to run, so we will just idle
            unsafe {
                arch::irq::enable();
                arch::irq::wait();
                arch::irq::disable();
            }
        }
    }

    pub fn schedule(&self) {
        let mut current = CURRENT
            .local()
            .lock()
            .take()
            .expect("Cannot schedule without a task");
        let next = self.pop_or_idle();

        let switch =
            arch::thread::prepare_switch(&mut current.thread.lock(), &mut next.thread.lock());

        // The task that was running is now ready to run again, and we need to
        // schedule the next task to run
        current.remaning = Wrapping(DEFAULT_QUANTUM);
        self.enqueue_ready(current);
        *CURRENT.local().lock() = Some(next);

        unsafe {
            arch::thread::perform_switch(switch);
        }
    }
}

#[derive(Debug)]
pub struct Task {
    thread: Arc<Spinlock<Thread>>,
    remaning: Wrapping<u64>,
}

impl Task {
    #[must_use]
    pub fn new(thread: Thread) -> Self {
        Self {
            thread: Arc::new(Spinlock::new(thread)),
            remaning: Wrapping(DEFAULT_QUANTUM),
        }
    }
}

pub fn tick() {
    let current = CURRENT.local();
    let mut current = current.lock();
    if let Some(task) = current.as_mut() {
        task.remaning -= 1;
        if task.remaning.0 == 0 {
            core::mem::drop(current);
            SCHEDULER.schedule();
        }
    }
}

pub fn enter() -> ! {
    let task = SCHEDULER.pop_or_idle();
    let jump = arch::thread::prepare_jump(&mut task.thread.lock());

    *CURRENT.local().lock() = Some(task);
    unsafe {
        arch::thread::perform_jump(jump);
    }
}
