#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    arch::lang::panic(info)
}
