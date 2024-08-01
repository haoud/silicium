use crate::{drivers, future};

/// Format a string and write it to the terminal.
#[macro_export]
macro_rules! print {
    ($tty:expr, $($arg:tt)*) => {
        $tty.write_str(alloc::format!($($arg)*).as_str()).await;
    };
}

/// Format a string and write it to the terminal, appending a newline.
#[macro_export]
macro_rules! println {
    ($tty:expr, $($arg:tt)*) => {
        $tty.write_str(alloc::format!($($arg)*).as_str()).await;
        $tty.write_str("\n").await;
    };
}

pub use print;
pub use println;

/// Returns the name of the month given the month number. If the month number
/// is invalid, it returns "Unknown" (i.e the month number is not in the range
/// 1-12).
pub fn month_name<T: Into<u8>>(month: T) -> &'static str {
    match month.into() {
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
    }
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
        print!(tty, "{i}... ");
        future::sleep::sleep(core::time::Duration::from_secs(1)).await;
    }
}
