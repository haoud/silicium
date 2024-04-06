use crate::future;

pub mod elf;
pub mod pid;
pub mod process;
pub mod syscall;
pub mod thread;
pub mod tid;

/// Create the init process.
///
/// # Safety
/// This function is unsafe because it must be  called during the initialization of the
/// kernel.
#[init]
pub unsafe fn setup() {
    let init = include_bytes!("../../../iso/boot/init.elf");
    let process = Arc::new(process::Process::new());
    let thread = elf::load(process.clone(), init).expect("failed to load init process");
    future::thread::spawn(thread);
}
