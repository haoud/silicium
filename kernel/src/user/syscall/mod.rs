use super::thread::Resume;
use crate::arch::x86_64::cpu::InterruptFrame;

/// The list of all syscalls numbers supported by the kernel.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Syscall {
    Exit = 0,
}

pub fn handler(frame: &mut InterruptFrame) -> Resume {
    // SAFETY: The syscall number is guaranteed to be an valid enum variant,
    // checking early during the syscall entry point, written in assembly.
    let syscall = unsafe { core::mem::transmute::<u32, Syscall>(frame.rax as u32) };

    match syscall {
        Syscall::Exit => Resume::Terminate(frame.rsi as u32),
    }
}
