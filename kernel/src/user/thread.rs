use time::Timespec;

use super::tid::Tid;
use crate::arch::{self, context::Context, paging::PageTable};

/// The base address of the stack of the thread. This is temporary and should be replaced
/// by a more dynamic solution in the future by allocating a virtual memory region for the
/// thread stack.
pub const STACK_BASE: usize = 0x0000_07FF_FFFF_F000;

/// Represents an user thread. A thread is a sequence of instructions that belongs to a
/// process and that can run concurrently with other threads in the system. Threads share
/// the same address space and resources as the process they belong to, but have their
/// own execution context, state and stack and may also have their own resources.
#[derive(Debug)]
pub struct Thread {
    /// The identifier of the thread
    tid: Tid,

    /// The state of the thread
    state: State,

    /// The context of the thread. This contains some architecture-specific data
    /// that is used to save and restore the state of the thread when it is scheduled.
    context: Context,

    /// The page table of the thread. This is used to map the virtual memory of the
    /// thread to the physical memory of the system.
    page_table: Arc<spin::Mutex<PageTable>>,

    /// The virtual runtime of the thread. This is used by the scheduler to determine
    /// the order in which threads should be scheduled.
    pub(super) vruntime: Timespec,

    /// The deadline of the thread. This is used by the scheduler to determine when
    /// the thread should be preempted if it has not finished executing.
    pub(super) deadline: Timespec,
}

impl Thread {
    /// # Panics
    /// Panics if the kernel ran out of TIDs
    #[must_use]
    pub fn new(entry: usize, page_table: Arc<spin::Mutex<PageTable>>) -> Self {
        Self {
            context: Context::new(entry, STACK_BASE),
            tid: Tid::generate().expect("kernel ran out of TIDs"),
            vruntime: Timespec::zero(),
            deadline: Timespec::zero(),
            state: State::Created,
            page_table,
        }
    }

    /// Get a reference to the page table of the thread
    #[must_use]
    pub fn page_table(&self) -> &Arc<spin::Mutex<PageTable>> {
        &self.page_table
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

    /// The thread has been killed by an signal and is waiting to be joined
    /// by another thread. This variant is similar to the `Exited` variant, but
    /// contains the signal that terminated the thread instead of the exit code.
    Killed(u32),
}

/// A trap that occurred during the execution of a thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Trap {
    /// An exception occurred during the execution of the thread.
    Exception,

    /// An interrupt occurred during the execution of the thread.
    Interrupt,

    /// A system call occurred during the execution of the thread.
    Syscall,
}

/// The behavior of the thread after a trap occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resume {
    /// Terminate the thread with the specified exit code. This means that the thread
    /// exited normally and will not be able to run again.
    Terminate(u32),

    /// Kill the thread with the specified signal. This means that the thread was killed
    /// for various reasons (illegal instruction, segmentation fault, signal sent by another
    /// thread, etc.) and will not be able to run again.
    Kill(u32),

    /// Continue the execution of the thread. This means that the thread will immediately be
    /// resumed and will continue to run until another trap occurs.
    Continue,

    /// Yield the thread. This means that the thread will be rescheduled and will be put back
    /// in the ready queue to be executed later.
    Yield,
}

/// Execute the thread. This function will jump to the thread's saved state and will
/// execute it until a trap occurs. The trap will be returned to the caller, which will
/// then decide what to do with the thread.
pub fn execute(thread: &mut Thread) -> Trap {
    // SAFETY: Changing page table should be safe
    unsafe {
        thread.page_table().lock().load_current();
    }
    arch::context::run(thread.context_mut())
}
