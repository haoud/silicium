use crate::arch::x86_64::cpu::InterruptFrame;

pub mod pid;
pub mod process;
pub mod scheduler;
pub mod sleep;
pub mod thread;
pub mod tid;

/// Automatically called by the interrupt handler when an user thread enters into
/// the kernel.
#[atomic]
#[no_mangle]
pub extern "C" fn kernel_enter(_frame: &mut InterruptFrame) {}

/// Automatically called by the interrupt handler when a user thread returns to
/// its code. This function will check if the current thread needs to be rescheduled
/// and if so, it will schedule the next thread.
#[atomic]
#[no_mangle]
pub extern "C" fn kernel_leave(frame: &mut InterruptFrame) {
    // Schedule the current thread if needed. If the current thread needs to be
    // rescheduled but no other threads are ready, we can just return to the
    // current thread and give it free extra quantum :)
    if scheduler::need_reschedule() {
        if let Some(next) = scheduler::pop() {
            scheduler::schedule_to(frame, next);
        }
    }
}
