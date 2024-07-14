use crate::{drivers, mm, time};
use core::fmt::Write;

/// The shell task.
///
/// Currently, it is not really a shell but a simple program that tests most
/// of the kernel cool features. It reads the keyboard input and converts it
/// to a character that is then written to the framebuffer.
pub async fn shell(mut tty: drivers::tty::VirtualTerminal<'_>) {
    tty.write_str("Silicium booted successfully\n");
    tty.write_str("Welcome to Silicium !\n");

    loop {
        write!(tty, "> ").unwrap();
        match tty.readline().await.as_str() {
            "meminfo\n" => {
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
            "date\n" => {
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
