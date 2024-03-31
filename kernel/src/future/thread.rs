use super::{executor, Task};
use crate::{
    arch::x86_64::idt::irq_handler,
    user::thread::{self, Thread},
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
        match thread::execute(&mut thread) {
            thread::Trap::Exception(_error, _id) => {
                irq_handler(thread.context_mut().registers_mut());
            }
            thread::Trap::Interrupt(_code) => {
                irq_handler(thread.context_mut().registers_mut());
            }
            thread::Trap::Syscall(_nr) => {
                log::debug!("Exiting");
            }
        };
    }
}
