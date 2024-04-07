use super::thread::{self, Resume, Thread};
use crate::arch;
use config::TIMER_HZ;
use time::{unit::Nanosecond, Timespec};

/// The global scheduler instance. This is responsible for managing the
/// ready queue and selecting the next thread to be executed by a CPU.
static SCHEDULER: spin::Mutex<FairScheduler> = spin::Mutex::new(FairScheduler::new());

/// The Fair Scheduler is a simple scheduler that uses a simplified version
/// of the Completely Fair Scheduler (CFS) algorithm, previously used in the
/// Linux kernel. The scheduler maintains a ready queue of threads, sorted
/// by their virtual runtime (vruntime). The thread with the lowest vruntime
/// is selected to be executed by a CPU core when another thread is preempted,
/// finishes executing, or yields the CPU.
///
/// # Possible Improvements
/// - Use a red-black tree to store the ready queue, allowing to insert and
///   remove threads in O(log n) time complexity, instead of O(n) with a vector.
/// - Use a per-CPU ready queue to reduce contention on the global ready queue,
///   and allowing a dynamic deadline calculation based on the number of threads
///   in the local ready queue.
/// - When a per-CPU ready queue will be implemented, the scheduler will need to
///   balance the load between the CPUs by moving threads between the ready queues
///   if the number of threads in one queue is significantly higher than the others.
///   This will prevent a single CPU from being overloaded while others are idle.
///   Running a load balancing algorithm periodically will help to ensure that the
///   load is evenly distributed between the CPUs. Every second should be enough.
#[derive(Debug)]
pub struct FairScheduler {
    /// The minimum vruntime of all threads in the ready queue. This is used
    /// to when adding a new thread to the ready queue to ensure that the new
    /// thread will not have a lower vruntime than the last thread in the queue,
    /// which would cause the new thread to be scheduled more often than others.
    min_vruntime: Timespec,

    /// The ready queue of threads. Threads are sorted by their vruntime, with
    /// the thread with the lowest vruntime being the last in the vector, allowing
    /// to simply pop the last thread to get the next thread to be executed.
    ready: Vec<Thread>,
}

impl FairScheduler {
    /// Create a new instance of the Fair Scheduler.
    #[must_use]
    pub const fn new() -> Self {
        FairScheduler {
            min_vruntime: Timespec::zero(),
            ready: Vec::new(),
        }
    }

    /// Add a thread to the ready queue. This will make the thread eligible
    /// for execution by the scheduler, and will set its state to `Ready`.
    pub fn add(&mut self, mut thread: Thread) {
        thread.vruntime = core::cmp::max(thread.vruntime, self.min_vruntime);
        thread.set_state(thread::State::Ready);
        self.ready.push(thread);
        self.ready
            .sort_by_key(|thread| core::cmp::Reverse(thread.vruntime));
    }
}

/// The entry point for the scheduler. This function will run indefinitely,
/// executing threads in the ready queue with the lowest vruntime.
///
/// We must ensure that this function is placed in the `.text` section, so
/// it will not included in the .init section. This is because the .init section
/// can be discarded after the kernel is initialized, and we need to keep this
/// function in memory indefinitely to run the scheduler.
#[link_section = ".text"]
pub fn enter() -> ! {
    loop {
        let mut thread = get_thread();
        while thread.state() == thread::State::Running {
            // Execute the thread until a trap occurs, and monitor the time the thread is
            // running to update the vruntime.
            // TODO: For AP cores, configure the timer to generate an interrupt when the
            // deadline will be reached
            let start = arch::time::current_timespec();
            let trap = thread::execute(&mut thread);
            let mut end = arch::time::current_timespec();

            let register = thread.context_mut().registers_mut();
            let resume = match trap {
                thread::Trap::Exception => arch::exception::handle(register),
                thread::Trap::Interrupt => arch::irq::handle(register),
                thread::Trap::Syscall => Resume::Continue,
            };

            // If the start time is greater than the end time, it means that the trap is
            // probably due to a timer interrupt. In this case, we should add one tick in
            // nanoseconds to the end time to correctly calculate the vruntime, because the
            // kernel tick count was not updated when calling `current_timespec`.
            if start > end {
                end.nano.0 += 1_000_000_000 / u64::from(TIMER_HZ);
            }
            thread.vruntime += end - start;

            match resume {
                Resume::Terminate(code) => {
                    log::info!("Thread {} terminated with code {}", thread.tid(), code);
                    thread.set_state(thread::State::Exited(code))
                }
                Resume::Kill(code) => {
                    log::info!("Thread {} killed with code {}", thread.tid(), code);
                    thread.set_state(thread::State::Killed(code))
                }
                Resume::Continue => {
                    // If the thread has not finished executing before the deadline, add it
                    // back to the ready queue.
                    if thread.deadline > thread.vruntime {
                        // TODO: Do not jump here if there is no other thread to run, or if the
                        // thread has still the lowest vruntime.
                        arch::context::save(thread.context_mut());
                        add_thread(thread);
                        break;
                    }
                }
                Resume::Yield => {
                    // Save some thread data that was not saved when ensuring the thread and add
                    // it back to the ready queue. In the next iteration, the scheduler will pick
                    // the thread with the lowest vruntime, i.e the thread that has been running
                    // for the least amount of time.
                    arch::context::save(thread.context_mut());
                    add_thread(thread);
                    break;
                }
            }
        }
    }
}

/// Get the next thread to be executed by the scheduler. This function will
/// block until a thread is available in the ready queue. During this time,
/// the CPU will be idle to save power.
pub fn get_thread() -> Thread {
    loop {
        // If there are no threads in the ready queue, wait for an interrupt
        // to occur. During this time, the CPU will be idle to save power.
        while SCHEDULER.lock().ready.is_empty() {
            unsafe {
                arch::irq::enable_and_wait();
                arch::irq::disable();
            }
        }

        // Get the next thread to be executed by the scheduler.
        let mut scheduler = SCHEDULER.lock();
        if let Some(mut thread) = scheduler.ready.pop() {
            // Update the minimum vruntime to the last thread in the ready
            // queue. This is needed because otherwise, a thread that has
            // been sleeping for a long time will have a disproportionately
            // low vruntime and will be scheduled more often than other threads.
            if let Some(last) = scheduler.ready.last() {
                scheduler.min_vruntime = last.vruntime;
            }

            // Set the deadline to 20ms from the current vruntime. If the thread
            // does not finish executing within this time, it will be preempted
            // and added back to the ready queue.
            thread.deadline = thread.vruntime + Nanosecond::new(20_000_000);
            thread.set_state(thread::State::Running);
            return thread;
        }

        // Oops! Our work was stolen by another thread. Let's try again...
    }
}

/// Add a thread to the ready queue. This will make the thread eligible
/// for execution by the scheduler.
pub fn add_thread(thread: Thread) {
    SCHEDULER.lock().add(thread);
}
