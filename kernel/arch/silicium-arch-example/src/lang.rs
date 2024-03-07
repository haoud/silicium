/// The panic handler for the kernel.
///
/// This function is called when the kernel panics and cannot recover. It should
/// disable interrupts on the current core, halt other cores, and then halt the
/// current core.
///
/// If the `panic_info` feature is enabled, this function should also log some
/// information about the panic, such as the message and location.
///
/// This function should be capable of handling recursive panics without rebooting.
/// In most cases, this means that the function should set a flag during the first
/// panic and then check for the flag during subsequent panics. If the flag is set,
/// the function should halt the CPU instead of trying to handle the panic.
#[cfg(not(test))]
#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
