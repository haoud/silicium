use crate::{arch, drivers, future, mm, time};
use core::fmt::Write;

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
        write!(tty, "> ").unwrap();
        match tty.readline().await.as_str() {
            "reboot" => {
                arch::x86_64::cpu::reboot();
                writeln!(tty, "Failed to reboot").unwrap();
            }
            "poweroff" => {
                arch::x86_64::cpu::poweroff();
                writeln!(tty, "Failed to poweroff").unwrap();
            }
            "meminfo" => {
                let total = mm::physical::STATE.lock().frames_info().len() * 4;
                let free = mm::physical::STATE
                    .lock()
                    .frames_info()
                    .iter()
                    .filter(|f| {
                        f.flags().contains(mm::physical::frame::Flags::FREE)
                    })
                    .count()
                    * 4;

                let kernel = mm::physical::STATE
                    .lock()
                    .frames_info()
                    .iter()
                    .filter(|f| {
                        f.flags().contains(mm::physical::frame::Flags::KERNEL)
                    })
                    .count()
                    * 4;

                let boot = mm::physical::STATE
                    .lock()
                    .frames_info()
                    .iter()
                    .filter(|f| {
                        f.flags().contains(mm::physical::frame::Flags::BOOT)
                    })
                    .count()
                    * 4;
                writeln!(tty, "Total: {} KiB ({} Mib)", total, total / 1024)
                    .unwrap();
                writeln!(tty, "Free: {} KiB ({} Mib)", free, free / 1024)
                    .unwrap();
                writeln!(tty, "Kernel: {} KiB ({} Mib)", kernel, kernel / 1024)
                    .unwrap();
                writeln!(tty, "Boot: {} KiB ({} Mib)", boot, boot / 1024)
                    .unwrap();
            }
            "date" => {
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
                    time.hour,
                    time.minute,
                    time.second,
                    time.day,
                    month,
                    time.year
                )
                .unwrap();
            }
            _ => {
                writeln!(tty, "Unknown command").unwrap();
            }
        }
    }
}
