use super::{process::Process, tid::Tid};
use crate::arch::context::Context;
use core::num::Saturating;

pub const STACK_BASE: usize = 0x0000_07FF_FFFF_F000;

/// An user thread.
#[derive(Debug)]
pub struct Thread {
    /// The identifier of the thread
    tid: Tid,

    /// The state of the thread
    state: State,

    /// The time slice of the thread in ticks
    quantum: Saturating<u64>,

    /// Whether the thread should be rescheduled or not
    reschedule: bool,

    /// The context of the thread. This contains some architecture-specific data
    /// that is used to save and restore the state of the thread when it is scheduled.
    context: Context,

    /// The process that the thread belongs to
    process: Arc<Process>,
}

impl Thread {
    /// # Panics
    /// Panics if the kernel ran out of TIDs
    #[must_use]
    pub fn new(process: Arc<Process>, entry: usize) -> Self {
        Self {
            context: Context::new(entry, STACK_BASE),
            process,
            tid: Tid::generate().expect("kernel ran out of TIDs"),
            state: State::Created,
            quantum: Saturating(0),
            reschedule: false,
        }
    }

    /// Get the process that the thread belongs to
    #[must_use]
    pub const fn process(&self) -> &Arc<Process> {
        &self.process
    }

    /// Get a mutable reference to the context of the thread
    #[must_use]
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    /// Get a reference to the context of the thread
    #[must_use]
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Return a mutable reference to the time slice of the thread
    #[must_use]
    pub const fn quantum_mut(&mut self) -> &mut Saturating<u64> {
        &mut self.quantum
    }

    /// Return a mutable reference to the time slice of the thread
    #[must_use]
    pub const fn quantum(&self) -> &Saturating<u64> {
        &self.quantum
    }

    /// Return whether the thread needs to be rescheduled or not
    #[must_use]
    pub fn needs_reschedule(&self) -> bool {
        self.reschedule
    }

    /// Set whether the thread needs to be rescheduled or not
    pub fn set_reschedule(&mut self, reschedule: bool) {
        self.reschedule = reschedule;
    }

    /// Set the state of the thread
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    /// Get the state of the thread
    #[must_use]
    pub fn state(&self) -> State {
        self.state
    }

    /// Get the identifier of the thread
    #[must_use]
    pub const fn tid(&self) -> &Tid {
        &self.tid
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        self.tid.deallocate();
    }
}

/// The state of a thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    /// The thread has been created but is not yet ready to run.
    Created,

    /// The thread is ready to run and is waiting to be scheduled by the kernel.
    Ready,

    /// The thread is currently running on the CPU.
    Running,

    /// The thread is currently sleeping and is waiting for a specific event to
    /// occur before being woken up. If an signal is sent to the thread, it will
    /// be woken up and will return to the `Ready` state, even if the event it
    /// was waiting for did not occur.
    Sleeping,

    /// The thread is currently waiting for an event to occur. If the event occurs,
    /// the thread will return to the `Ready` state, otherwise it will remain in
    /// this state until the event occurs. This state is similar to the `Sleeping`
    /// state, but the thread cannot be woken up by a signal.
    Waiting,

    /// The thread has exited and is waiting to be joined by another thread. This
    /// variant contains the exit code of the thread.
    Exited(u32),

    /// The thread has been terminated by an signal and is waiting to be joined
    /// by another thread. This variant is similar to the `Exited` variant, but
    /// contains the signal that terminated the thread instead of the exit code.
    Terminated(u32),
}
