#![no_std]
#![no_main]

#[no_mangle]
pub unsafe fn _start() -> ! {
    loop {
        core::arch::asm!("syscall");
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
