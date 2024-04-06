use crate::arch::x86_64::cpu::InterruptFrame;

#[no_mangle]
pub extern "C" fn syscall_handler(frame: &mut InterruptFrame) {
    log::debug!("Syscall: {}", frame.rax);
}
