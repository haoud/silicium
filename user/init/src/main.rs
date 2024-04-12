#![no_std]
#![no_main]

#[no_mangle]
pub unsafe fn _start() -> ! {
    loop {
        core::arch::asm!("mov rax, 2");
        core::arch::asm!("xor rsi, rsi");
        core::arch::asm!("syscall");
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
