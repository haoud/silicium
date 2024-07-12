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

    // Setup the time system
    time::setup();

    // Setup the async runtime
    future::setup();

    // Setup the keyboard driver
    drivers::kbd::setup();

    // Setup the framebuffer
    let fb = drivers::fb::setup();
    let tty = drivers::tty::VirtualTerminal::new(fb);

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
    use futures::StreamExt;
    use pc_keyboard::{DecodedKey, KeyState, ScancodeSet, ScancodeSet1};

    let mut kbd = drivers::kbd::KeyboardStream::new();
    let mut decoder = pc_keyboard::EventDecoder::new(
        pc_keyboard::layouts::Azerty,
        pc_keyboard::HandleControl::Ignore,
    );
    let mut set = ScancodeSet1::new();

    tty.write_str("Silicium booted successfully\n");
    tty.write_str("Welcome to Silicium !\n");
    tty.flush();

    loop {
        // Wait for the next scancode
        let scancode = match kbd.next().await {
            Some(scancode) => scancode,
            None => continue,
        };

        // Advance the keyboard state and get the key from the scancode
        let key = set
            .advance_state(scancode)
            .expect("Failed to advance the keyboard state")
            .unwrap();

        // If the key is pressed, write the character to the framebuffer
        // and flush it to redraw the cursor.
        if key.state == KeyState::Down {
            let char = match decoder.process_keyevent(key) {
                Some(DecodedKey::Unicode(c)) => c,
                _ => continue,
            };
            tty.write(char);
            tty.flush();
        }
    }
}
