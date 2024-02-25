/// Halt the current CPU forever
#[inline]
pub fn halt() -> ! {
    arch::cpu::halt();
}
