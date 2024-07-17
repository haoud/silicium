use crate::{arch, drivers, future, mm, time};
use alloc::format;
use core::fmt::Write;

/// Format a string and write it to the terminal.
macro_rules! print {
    ($tty:expr, $($arg:tt)*) => {
        $tty.write_str(format!($($arg)*).as_str()).await;
    };
}

/// Format a string and write it to the terminal, appending a newline.
macro_rules! println {
    ($tty:expr, $($arg:tt)*) => {
        $tty.write_str(format!($($arg)*).as_str()).await;
        $tty.write_str("\n").await;
    };
}

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
    countdown(tty, "Powering off", 5).await;
    arch::x86_64::cpu::poweroff();
    println!(tty, "Failed to power off !");
}

pub async fn reboot(tty: &mut drivers::tty::VirtualTerminal) {
    countdown(tty, "Rebooting", 5).await;
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

    // FIXME: The memory used by the boot allocator is not accounted for
    // kernel memory usage. Fix this ASAP.
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
    let month = match time.month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    };
    writeln!(
        tty,
        "Date: {}:{}:{} {} {} {}\n",
        time.hour, time.minute, time.second, time.day, month, time.year
    )
    .unwrap();
}

/// Countdown from `secs` to 0, printing the current number to the terminal
/// every second. The `event` string will be printed before the countdown.
pub async fn countdown(
    tty: &mut drivers::tty::VirtualTerminal,
    event: &str,
    secs: u64,
) {
    print!(tty, "{event} in ");
    for i in (1..=secs).rev() {
        write!(tty, "{i}... ").unwrap();
        future::sleep::sleep(core::time::Duration::from_secs(1)).await;
    }
}
