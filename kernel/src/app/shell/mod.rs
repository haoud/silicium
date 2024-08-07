use crate::{arch, drivers, future, mm, time};
use core::fmt::Write;
use utils::{print, println};

pub mod utils;

/// Setup the terminal. This function will initialize the terminal if a
/// framebuffer is available. The terminal will use the framebuffer as the
/// output device and the keyboard as the input device.
///
/// If no framebuffer is available, this function will do nothing but log a
/// warning indicating that the terminal will not be initialized.
pub fn setup() {
    if drivers::fb::FRAMEBUFFER.lock_blocking().is_valid() {
        let kbd = drivers::kbd::KeyboardScancodeStream::new();
        let stream = drivers::tty::input::KeyboardCharStream::new(kbd);
        let input = drivers::tty::input::TerminalInput::new(Box::pin(stream));

        future::executor::schedule_detached(shell(future::executor::block_on(
            drivers::tty::VirtualTerminal::new(
                Arc::clone(&drivers::fb::FRAMEBUFFER),
                input,
            ),
        )));
    } else {
        log::warn!("No framebuffer available, terminal not initialized");
    }
}

/// The shell task.
///
/// Currently, it is not really a shell but a simple program that tests most
/// of the kernel cool features. It reads the keyboard input and converts it
/// to a character that is then written to the framebuffer.
pub async fn shell(mut tty: drivers::tty::VirtualTerminal) {
    tty.write_str("Silicium booted successfully\n").await;
    tty.write_str("Welcome to Silicium !\n").await;

    loop {
        print!(tty, "> ");
        let input = tty.readline().await;
        let arg = input.split_whitespace().collect::<Vec<_>>();

        // If no input was provided, skip the rest of the loop and
        // prompt the user again.
        if input.is_empty() {
            continue;
        }

        match arg[0] {
            "reboot" => {
                reboot(&mut tty).await;
            }
            "poweroff" => {
                poweroff(&mut tty).await;
            }
            "clear" => {
                tty.clear().await;
            }
            "meminfo" => {
                meminfo(&mut tty).await;
            }
            "date" => {
                date(&mut tty);
            }
            _ => {
                writeln!(tty, "Unknown command").unwrap();
            }
        }
    }
}

pub async fn poweroff(tty: &mut drivers::tty::VirtualTerminal) {
    utils::countdown(tty, "Powering off", 5).await;
    arch::x86_64::cpu::poweroff();
    println!(tty, "Failed to power off !");
}

pub async fn reboot(tty: &mut drivers::tty::VirtualTerminal) {
    utils::countdown(tty, "Rebooting", 5).await;
    arch::x86_64::cpu::reboot();
    println!(tty, "Failed to reboot !");
}

pub async fn meminfo(tty: &mut drivers::tty::VirtualTerminal) {
    let total = mm::physical::STATE.lock().frames_info().len() * 4;
    let free = mm::physical::STATE
        .lock()
        .frames_info()
        .iter()
        .filter(|f| f.flags().contains(mm::physical::frame::Flags::FREE))
        .count()
        * 4;

    let kernel = mm::physical::STATE
        .lock()
        .frames_info()
        .iter()
        .filter(|f| f.flags().contains(mm::physical::frame::Flags::KERNEL))
        .count()
        * 4;

    println!(
        tty,
        "{}/{} KiB ({}%) used",
        total - free,
        total,
        (total - free) * 100 / total
    );

    println!(
        tty,
        "{} KiB used by kernel ({}%)",
        kernel,
        kernel * 100 / total
    );
}

pub fn date(tty: &mut drivers::tty::VirtualTerminal) {
    let time = time::Date::from(time::Unix::current());
    let month = utils::month_name(time.month);
    writeln!(
        tty,
        "Date: {:02}:{:02}:{:02} {} {} {}\n",
        time.hour, time.minute, time.second, time.day, month, time.year
    )
    .unwrap();
}
