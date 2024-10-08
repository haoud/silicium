#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(const_mut_refs)]
#![feature(negative_impls)]
#![feature(prelude_import)]
#![feature(const_option)]
#![feature(step_trait)]
#![feature(new_uninit)]
#![allow(internal_features)]

extern crate alloc;

pub mod app;
pub mod arch;
pub mod boot;
pub mod drivers;
pub mod future;
pub mod library;
pub mod mm;
pub mod prelude;
pub mod time;

#[allow(unused_imports)]
#[prelude_import]
pub use prelude::*;

/// The entry point for the kernel. This function call the architecture
/// specific setup function, print a message to the console and then halts
/// the CPU.
///
/// # Safety
/// This function is marked as unsafe because it must be called only once
/// at the start of the kernel. Failing to do so will result in undefined
/// behavior.
#[init]
#[no_mangle]
pub unsafe extern "C" fn _entry() -> ! {
    // Call the architecture specific setup function
    let info = arch::entry();

    // Setup the memory management system
    mm::setup(&info);

    // Setup the architecture specific late setup that needs the
    // memory management system to be setup first
    arch::setup();

    // Setup the time system
    time::setup();

    // Setup the async runtime
    future::setup();

    // Setup the keyboard driver
    drivers::kbd::setup();

    // Setup the framebuffer driver
    drivers::fb::setup();

    // Setup the terminal.
    app::shell::setup();

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully !");

    // TODO: Use a more reliable stack (this stack will be deallocated in
    // the future because it is marked as boot reclaimable since it is
    // provided by the bootloader)
    future::executor::run();
}
