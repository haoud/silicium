/// Halt the current CPU forever
#[cold]
pub fn halt() -> ! {
    arch::cpu::halt();
}
