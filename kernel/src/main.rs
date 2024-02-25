#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

/// The entry point for the kernel. This function call the architecture specific setup
/// function, print a message to the console and then halts the CPU.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Call the architecture specific setup function
    arch::setup();

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully");

    // Halt the CPU
    arch::cpu::halt();
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
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
