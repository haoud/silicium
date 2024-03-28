use super::thread::{State, Thread};
use crate::arch::{self, context::Registers};
use alloc::collections::VecDeque;
use core::ops::SubAssign;
use spin::Spinlock;

/// The current thread running on the current core. If `None`, the core has no
/// current thread and is idle.
#[per_cpu]
pub static CURRENT: Spinlock<Option<Thread>> = Spinlock::new(None);

/// The scheduler for the kernel
static SCHEDULER: Spinlock<Scheduler> = Spinlock::new(Scheduler::new());

/// The time quantum for each thread in ticks
const QUANTUM: u64 = 10;

/// A simple round-robin scheduler
#[derive(Debug)]
pub struct Scheduler {
    ready: VecDeque<Thread>,
}

impl Scheduler {
    /// Create a new scheduler with an empty ready queue
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ready: VecDeque::new(),
        }
    }

    /// Push a thread to the back of the ready queue, and set its quantum to the
    /// default value and its state to `State::Ready`.
    pub fn enqueue(&mut self, mut thread: Thread) {
        thread.quantum_mut().0 = QUANTUM;
        thread.set_state(State::Ready);
        self.ready.push_back(thread);
    }

    /// Pop a thread from the front of the ready queue and set its state to
    /// `State::Running`. If the queue is empty, return `None`.
    #[must_use]
    pub fn pop(&mut self) -> Option<Thread> {
        self.ready.pop_front().map(|mut thread| {
            thread.set_state(State::Running);
            thread
        })
    }
}

/// Add a thread to the scheduler's ready queue.
pub fn add(thread: Thread) {
    SCHEDULER.lock().enqueue(thread);
}

/// Get a thread ready to run. If none is available, idle the CPU until one
/// becomes available.
#[must_use]
pub fn pop_or_idle() -> Thread {
    loop {
        if let Some(thread) = pop() {
            return thread;
        }

        // TODO: Try to steal async task work from other CPUs
        // SAFETY: This is safe because we only enable interrupts in a very
        // short section of code simply to be woken up by an interrupt.
        unsafe {
            let irq = arch::irq::save();
            arch::irq::enable();
            arch::irq::wait();
            arch::irq::restore(irq);
        }
    }
}

/// Get a thread ready to run. Returns `None` if no thread is available.
#[must_use]
pub fn pop() -> Option<Thread> {
    SCHEDULER.lock().pop()
}

/// Called by the timer interrupt to decrement the quantum of the current thread.
/// If the quantum reaches zero, the thread is marked for rescheduling and will be
/// rescheduled at the next opportunity (e.g., before returning to user mode).
pub fn tick() {
    if let Some(current) = CURRENT.local().lock().as_mut() {
        current.quantum_mut().sub_assign(1);
        if current.quantum().0 == 0 {
            current.set_reschedule(true);
        }
    }
}

/// Schedule the next thread to run. The registers passed as parameter will be
/// saved in the current thread before switching to the next thread.
#[atomic]
pub fn schedule_to(regs: &Registers, next: Thread) {
    let mut current = CURRENT
        .local()
        .lock()
        .take()
        .expect("Cannot reschedule without a current thread");

    let state = current.state();
    let current_context = current.context_mut();
    let next_context = next.context();

    unsafe {
        next.process().page_table().lock().load_current();
    }

    let switch = match state {
        State::Terminated(_) | State::Exited(_) => {
            todo!("Handle terminated or killed threads")
        }
        State::Sleeping | State::Waiting => {
            todo!("Handle sleeping or waiting threads")
        }
        State::Running => {
            let switch = arch::context::prepare_switch(regs, current_context, next_context);
            SCHEDULER.lock().enqueue(current);
            switch
        }
        _ => unreachable!("Invalid state for scheduling a thread"),
    };

    unsafe {
        arch::context::perform_switch(switch);
    }
}

/// Enter user mode by jumping to a thread's context. This function should only
/// be called when the kernel has finished its initialization and is ready to
/// start running user code.
///
/// If there is no thread to run, the CPU will be idle until a thread becomes
/// available.
pub fn enter_usermode() -> ! {
    let next = pop_or_idle();
    let jump = arch::context::prepare_jump(next.context());

    unsafe {
        next.process().page_table().lock().load_current();
    }

    set_current(next);
    unsafe {
        arch::context::perform_jump(jump);
    }
}

/// Verify if the current thread needs to be rescheduled or not
#[must_use]
pub fn need_reschedule() -> bool {
    CURRENT
        .local()
        .lock()
        .as_ref()
        .map_or(false, |current| current.needs_reschedule())
}

/// Set the current thread for the current core
fn set_current(thread: Thread) {
    CURRENT.local().lock().replace(thread);
}
