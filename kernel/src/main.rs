#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(const_mut_refs)]

extern crate alloc;

#[cfg(not(test))]
pub mod lang;
pub mod mm;

/// The entry point for the kernel. This function call the architecture specific setup
/// function, print a message to the console and then halts the CPU.
///
/// # Safety
/// This function is marked as unsafe because it must be called only once at the start
/// of the kernel. Failing to do so will result in undefined behavior.
#[no_mangle]
#[macros::init]
#[cfg(not(test))]
pub unsafe extern "C" fn _start() -> ! {
    // Call the architecture specific setup function
    let info = arch::setup();

    // Setup the memory management system
    mm::setup(&info);

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully");

    // Enable interrupts and wait for them
    arch::irq::enable();
    loop {
        arch::irq::wait();
    }
}
