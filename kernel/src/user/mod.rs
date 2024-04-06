pub mod elf;
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
    let thread = elf::load(init).expect("failed to load init process");
}
