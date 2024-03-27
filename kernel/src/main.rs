#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(negative_impls)]
#![feature(prelude_import)]
#![feature(new_uninit)]
#![feature(const_for)]
#![feature(effects)]
#![allow(internal_features)]

extern crate alloc;

pub mod arch;
pub mod boot;
pub mod future;
pub mod mm;
pub mod preempt;
pub mod prelude;
pub mod time;
pub mod user;

#[allow(unused_imports)]
#[prelude_import]
pub use prelude::*;

/// The entry point for the kernel. This function call the architecture specific setup
/// function, print a message to the console and then halts the CPU.
///
/// # Safety
/// This function is marked as unsafe because it must be called only once at the start
/// of the kernel. Failing to do so will result in undefined behavior.
#[init]
#[no_mangle]
#[cfg(not(test))]
pub unsafe extern "C" fn _start() -> ! {
    // Call the architecture specific setup function
    let info = arch::setup();

    // Setup the memory management system
    mm::setup(&info);

    // Setup the time system
    time::setup();

    // Setup the async runtime
    future::setup();

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully");

    user::scheduler::enter_usermode();
}
