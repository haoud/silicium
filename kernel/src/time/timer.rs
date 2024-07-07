use alloc::collections::BinaryHeap;

use crate::{library::spin::Spinlock, time::instant::Instant};
use core::{cmp::Reverse, task::Waker};

/// The list of active timers. This is a priority queue that is sorted by the
/// deadline of the timers. The first timer in the queue is the one that will
/// expire the soonest.
static TIMERS: Spinlock<BinaryHeap<Reverse<Timer>>> =
    Spinlock::new(BinaryHeap::new());

/// A timer that will expire at a given deadline and wake up a task.
#[derive(Debug)]
pub struct Timer {
    /// The deadline of the timer. The timer will expire when the current
    /// time is greater than or equal to the deadline.
    deadline: Instant,

    /// The waker that will be used to wake up the task when the timer expires.
    waker: Box<Waker>,
}

impl Timer {
    /// Register a new timer that will wake up the task when the deadline is
    /// reached. If the deadline is already expired, the task will be woken up
    /// immediately and no timer will be registered.
    pub fn register(deadline: Instant, waker: Box<Waker>) {
        if deadline <= Instant::now() {
            waker.wake();
        } else {
            TIMERS
                .lock_irq_safe()
                .push(Reverse(Timer { deadline, waker }));
        }
    }

    /// Returns true if the timer has expired.
    #[must_use]
    pub fn expired(&self) -> bool {
        self.deadline <= Instant::now()
    }
}

impl PartialOrd for Timer {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.deadline.cmp(&other.deadline))
    }
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline
    }
}

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deadline.cmp(&other.deadline)
    }
}

impl Eq for Timer {}

/// Execute all expired timers and remove them from the list of active
/// timers.
pub fn handle() {
    let mut timers = TIMERS.lock_irq_safe();
    while let Some(timer) = timers.peek() {
        if timer.0.expired() {
            let timer = timers.pop().unwrap();
            timer.0.waker.wake();
        } else {
            break;
        }
    }
}
