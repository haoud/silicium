#![cfg_attr(not(test), no_std)]
#![no_main]

/// The entry point for the kernel
///
/// # Safety
/// This function is unsafe because at this point, the kernel is not yet initialized
/// and any code that runs here must be very careful to not cause undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
