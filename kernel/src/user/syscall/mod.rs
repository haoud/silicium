use super::thread::Resume;
use crate::arch::x86_64::cpu::InterruptFrame;

pub fn handler(frame: &mut InterruptFrame) -> Resume {
    let sys_nr = frame.rax as u32;

    match sys_nr {
        0 => Resume::Terminate(frame.rsi as u32),
        _ => Resume::Kill(frame.rax as u32),
    }
}
