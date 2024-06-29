#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(negative_impls)]
#![feature(prelude_import)]
#![allow(internal_features)]

// TODO::
// - Reduce the number of crates by integrating some of them into the
//   kernel crate. This will simply the code when a crate will depend
//   on a feature of the kernel crate, thus creating a circular dependency...
// - The APIC timer calibration is not very precise and can sometimes be
//   off by a factor of 10 !! Probably because it use the PIT timer to
//   calibrate the APIC timer. We should find a way to calibrate the APIC
//   timer without using the PIT timer.

extern crate alloc;

pub mod arch;
pub mod boot;
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
    let info = arch::setup();

    // Setup the memory management system
    mm::setup(&info);

    // Setup the time system
    time::setup();

    // Setup the async runtime
    future::setup();

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully");

    // FIXME: Use a more reliable stack (this stack will be
    // deallocated in the future)
    future::executor::run();
}
