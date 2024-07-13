#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(negative_impls)]
#![feature(prelude_import)]
#![feature(const_option)]
#![feature(step_trait)]
#![allow(internal_features)]

extern crate alloc;

pub mod arch;
pub mod boot;
pub mod drivers;
pub mod future;
pub mod library;
pub mod mm;
pub mod prelude;
pub mod time;

#[allow(unused_imports)]
#[prelude_import]
pub use prelude::*;

/// The entry point for the kernel. This function call the architecture
/// specific setup function, print a message to the console and then halts
/// the CPU.
///
/// # Safety
/// This function is marked as unsafe because it must be called only once
/// at the start of the kernel. Failing to do so will result in undefined
/// behavior.
#[init]
#[no_mangle]
pub unsafe extern "C" fn _entry() -> ! {
    // Call the architecture specific setup function
    let info = arch::setup();

    // Setup the memory management system
    mm::setup(&info);

    // Setup the architecture specific late setup that needs the
    // memory management system to be setup first
    arch::late_setup();

    // Setup the time system
    time::setup();

    // Setup the async runtime
    future::setup();

    // Setup the keyboard driver
    drivers::kbd::setup();

    // Setup the terminal driver. It needs the framebuffer to be initialized
    // and will create an input stream from the keyboard driver.
    let fb = drivers::fb::setup();
    let kbd = drivers::kbd::KeyboardScancodeStream::new();
    let stream = drivers::tty::input::KeyboardCharStream::new(kbd);
    let input = drivers::tty::input::TerminalInput::new(Box::pin(stream));
    let tty = drivers::tty::VirtualTerminal::new(fb, input);

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully !");

    // TODO: Use a more reliable stack (this stack will be
    // deallocated in the future)
    future::executor::spawn(future::Task::new(shell(tty)));
    future::executor::run();
}

/// The shell task.
///
/// Currently, it is not really a shell but a simple program that tests most
/// of the kernel cool features. It reads the keyboard input and converts it
/// to a character that is then written to the framebuffer.
pub async fn shell(mut tty: drivers::tty::VirtualTerminal<'_>) {
    use core::fmt::Write;

    tty.write_str("Silicium booted successfully\n");
    tty.write_str("Welcome to Silicium !\n");

    loop {
        write!(tty, "> ").unwrap();
        let line = tty.readline().await;
        write!(tty, "You typed: {}", line).unwrap();
    }
}
