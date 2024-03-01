#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use macros::init;

/// The entry point for the kernel. This function call the architecture specific setup
/// function, print a message to the console and then halts the CPU.
///
/// # Safety
/// This function is marked as unsafe because it must be called only once at the start
/// of the kernel. Failing to do so will result in undefined behavior.
#[cfg(not(test))]
#[no_mangle]
#[init]
pub unsafe extern "C" fn _start() -> ! {
    // Call the architecture specific setup function
    arch::setup();

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully");

    // Enable interrupts and wait for them
    arch::irq::enable();
    loop {
        arch::irq::wait();
    }
}

#[cfg(test)]
pub mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
