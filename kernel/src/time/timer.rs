use crate::arch;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Spinlock;
use time::Timespec;

/// The list of active timers.
static TIMERS: Spinlock<Vec<Timer>> = Spinlock::new(Vec::new());

/// The callback type for timers.
type Callback = Box<dyn FnMut(&mut Timer) + Send>;

/// A timer that will execute a callback when it expires if the guard is still active.
pub struct Timer {
    callback: Option<Callback>,
    deadline: Timespec,
    guard: Guard,
}

impl Timer {
    /// Creates a new timer that will expire at the given time and will invoke the given
    /// callback. The expiration time is expressed in nanoseconds after the system was
    /// booted.
    /// It returns a guard that will cancel the timer when dropped if the `ignore`
    /// method was not called on the guard before the timer expiration.
    #[must_use]
    pub fn register<T>(deadline: Timespec, callback: T) -> Guard
    where
        T: FnMut(&mut Timer) + Send + 'static,
    {
        let guard = Guard {
            active: Arc::new(AtomicBool::new(true)),
            ignore: false,
        };

        let timer = Timer {
            callback: Some(Box::new(callback)),
            guard: guard.clone(),
            deadline,
        };

        timer.activate();
        guard
    }

    /// Returns true if the timer has expired.
    #[must_use]
    pub fn expired(&self) -> bool {
        self.deadline <= arch::time::current_timespec()
    }

    /// Returns true if the timer is active. If the timer was deactivated by a guard
    /// drop, this will return false.
    #[must_use]
    pub fn active(&self) -> bool {
        self.guard.active()
    }

    /// Activates the timer. If the timer has expired, it will be executed immediately,
    /// otherwise it will be pushed to the active timers list.
    fn activate(mut self) {
        if self.expired() {
            self.execute();
        } else {
            TIMERS.lock().push(self);
        }
    }

    /// Executes the timer callback if the timer guard is still active.
    ///
    /// # Panics
    /// Panics if the timer callback has already been called.
    fn execute(&mut self) {
        let mut callback = self.callback.take().expect("Timer callback already called");
        if !self.guard.ignore {
            (callback)(self);
        }
    }
}

/// A guard that will cancel the timer when dropped. It can be cloned to create multiple
/// guards that will all cancel the timer when dropped. If one guard is dropped, the
/// corresponding timer will be cancelled even if multiple guards are still active.
#[derive(Debug, Clone)]
pub struct Guard {
    /// The atomic boolean that will be set to false when the timer is cancelled. It is
    /// shared with the timer and with all the guards that have been cloned from the
    /// original guard.
    active: Arc<AtomicBool>,

    /// Set to true when the guard shoud be ignored when dropped.
    ignore: bool,
}

impl Guard {
    /// Returns `true` if the timer is active.
    #[must_use]
    pub fn active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    /// Ignore the guard when dropped: the timer will not be cancelled when
    /// this guard will be dropped.
    pub fn ignore(&mut self) {
        self.ignore = true;
    }

    /// Cancels the timer. The timer will not be executed when it expires. If the timer
    /// callback has already been called, this will have no effect.
    pub fn cancel(&self) {
        self.active.store(false, Ordering::Relaxed);
    }
}

impl Drop for Guard {
    /// When a guard is dropped, it will cancel the timer it is guarding if
    /// the ignore flag is not set in the current guard.
    fn drop(&mut self) {
        if !self.ignore {
            self.cancel();
        }
    }
}

/// Eecute all expired timers and remove them from the list of active timers. Inactive
/// timers will also be removed.
#[atomic]
pub fn handle() {
    let mut timers = TIMERS.lock();

    // Execute all expired timers and then remove them from the list of active
    // timers. Inactive timers will also be removed.
    timers
        .iter_mut()
        .filter(|timer| timer.expired())
        .for_each(|timer| timer.execute());

    timers.retain(|timer| !timer.expired() && timer.active());
}
