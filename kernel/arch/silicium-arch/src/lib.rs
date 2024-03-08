#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "panic_info", feature(panic_info_message))]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

// TMP
pub unsafe fn test() {
    let mut thread = arch::thread::Thread::kernel(hello);
    let register = thread.kstack().registers();
    arch::thread::prepare_jump(&mut thread);
    arch::thread::perform_jump(register);
}

fn hello() -> ! {
    let mut i = 0;
    unsafe {
        arch::irq::enable();
    }
    loop {
        arch::irq::wait();
        i += 1;
        if i % 1000 == 100 {
            ::log::info!("Hello, world!");
        }
    }
}
