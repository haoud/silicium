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
pub mod scheduler;
pub mod sys;
pub mod time;

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

    // Setup the scheduler
    scheduler::setup();

    // Setup the async runtime
    future::setup();

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully");

    let mut executor = future::Executor::new();
    executor.spawn(future::Task::new(test()));
    executor.spawn(future::Task::new(another_test()));

    arch::irq::enable();
    loop {
        executor.run_once();
    }

    // Enter the scheduler
    scheduler::enter();
}

async fn test() {
    loop {
        log::info!("Tic");
        crate::future::sleep::sleep(::time::unit::Nanosecond(2_000_000_000)).await;
    }
}

async fn another_test() {
    loop {
        crate::future::sleep::sleep(::time::unit::Nanosecond(1_000_000_000)).await;
        log::info!("Tac");
        crate::future::sleep::sleep(::time::unit::Nanosecond(1_000_000_000)).await;
    }
}
