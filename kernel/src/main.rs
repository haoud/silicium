#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]

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

    // Halt the CPU
    arch::cpu::halt();
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    arch::irq::disable();
    // TODO: Halt other cores

    log::error!("The kernel has encountered a fatal error that it cannot recover from");
    log::error!("The kernel must stop to prevent further damage");

    if let Some(message) = info.message() {
        if let Some(location) = info.location() {
            log::error!("{} at {}@CPU 0", message, location);
        } else {
            log::error!("{}", message);
        }
    }

    // Halt the CPU
    arch::cpu::halt();
}

#[cfg(test)]
pub mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
