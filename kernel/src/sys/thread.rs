use super::tid::Tid;
use crate::arch::context::Context;

/// A thread.
#[derive(Debug)]
pub struct Thread {
    /// The identifier of the thread
    tid: Tid,

    /// The process that contains the thread
    /// process: Weak<Spinlock<Process>>,

    /// The context of the thread. This contains some architecture-specific data
    /// that is used to save and restore the state of the thread when it is scheduled.
    context: Context,

    /// The state of the thread
    state: State,
}

impl Thread {
    /// Create a new kernel thread that will run the given function
    #[must_use]
    pub fn kernel(f: fn() -> !) -> Self {
        let tid = Tid::generate().expect("Failed to generate a new thread ID");
        let context = Context::kernel(f);
        let state = State::Created;

        Self {
            context,
            state,
            tid,
        }
    }

    /// Get the identifier of the thread
    #[must_use]
    pub const fn tid(&self) -> &Tid {
        &self.tid
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

    /// Get the state of the thread
    #[must_use]
    pub fn state(&self) -> State {
        self.state
    }

    /// Set the state of the thread
    pub fn set_state(&mut self, state: State) {
        self.state = state;
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
