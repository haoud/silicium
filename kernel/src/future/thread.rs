use super::{executor, task, Task};
use crate::{
    arch::x86_64::{exception, irq},
    user::thread::{self, Resume, Thread},
};

/// Spawn a thread. This will allow the thread to be executed by the executor.
pub fn spawn(thread: Thread) {
    executor::spawn(Task::new(thread_loop(thread)));
}

/// TODO: Explain this beautiful function !
pub async fn thread_loop(mut thread: Thread) {
    loop {
        // Jump to the thread's, and wait for a trap to occur
        // Handle the trap
        // Depending on the trap result:
        //  - Continue the execution of the thread
        //  - Yield the thread
        //  - Terminate the thread
        let resume = match thread::execute(&mut thread) {
            thread::Trap::Exception(error, id) => {
                let register = thread.context_mut().registers_mut();
                exception::handler(id, error, register)
            }
            thread::Trap::Interrupt(code) => {
                let thread = &mut thread;
                irq::user_handler(thread, code)
            }
            thread::Trap::Syscall(nr) => Resume::Terminate(nr),
        };

        // If the thread quantity is zero, yield the thread
        if thread.needs_reschedule() && resume == Resume::Continue {
            thread.set_reschedule(false);
            thread.restore_quantum();
            task::yield_now().await;
        } else if let Resume::Terminate(code) = resume {
            log::debug!("Thread terminated with exit code {}", code);
            break;
        } else if let Resume::Kill(code) = resume {
            log::debug!("Thread killed with exit code {}", code);
            break;
        } else {
            // We can safely continue the execution of the thread
        }
    }
}
