use crate::{arch, library::seq::Seqlock};
use macros::init;

pub mod instant;
pub mod timer;

/// Number of days elased since the beginning of the year, excluding the
/// current month.
const ELAPSED_DAYS_MONTHS: [usize; 12] =
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// The date at which the kernel was started.
static STARTUP_DATE: Seqlock<Date> = Seqlock::new(Date::epoch());

/// The Unix time at which the kernel was started.
static STARTUP_TIME: Seqlock<Unix> = Seqlock::new(Unix::epoch());

/// Represents a date in the Gregorian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// Represents the unix time, which is the number of seconds elapsed since
/// January 1st, 1970.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unix(pub u64);

impl Unix {
    #[must_use]
    pub const fn new(seconds: u64) -> Self {
        Self(seconds)
    }

    /// Get the Unix epoch, which is January 1st, 1970 at 00:00:00.
    #[must_use]
    pub const fn epoch() -> Self {
        Self(0)
    }

    /// Get the Unix time at which the kernel was started.
    #[must_use]
    pub fn boot() -> Self {
        STARTUP_TIME.read()
    }

    /// Get the current Unix time using the kernel startup time and the
    /// number of jiffies elapsed since the kernel was started.
    #[must_use]
    pub fn current() -> Self {
        let startup_time = STARTUP_TIME.read();
        let jiffies = arch::time::get_jiffies();
        let jiffies_frequency = arch::time::jiffies_frequency();
        Self::new(startup_time.0 + (jiffies / jiffies_frequency))
    }
}

impl From<Unix> for u64 {
    fn from(unix: Unix) -> Self {
        unix.0
    }
}

impl From<Date> for Unix {
    fn from(date: Date) -> Self {
        date.to_unix_time()
    }
}

impl Date {
    /// Get the Unix epoch, which is January 1st, 1970 at 00:00:00.
    #[must_use]
    pub const fn epoch() -> Self {
        Self {
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }

    /// Check if the given year is a leap year or not.
    #[must_use]
    pub const fn leap_year(years: u64) -> bool {
        years % 4 == 0 && (years % 100 != 0 || years % 400 == 0)
    }

    /// Converts the date to a Unix time. If the date is before January 1st,
    /// 1970, the Unix time returned will be the Unix epoch (January 1st, 1970
    /// at 00:00:00). If the year if greater than 2100, this function will
    /// also return the Unix epoch.
    #[must_use]
    pub fn to_unix_time(&self) -> Unix {
        if self.year < 1970 || self.year > 2100 {
            return Unix::epoch();
        }

        let mut seconds = u64::from(self.second);
        seconds += u64::from(self.minute) * 60;
        seconds += u64::from(self.hour) * 3600;
        seconds += (u64::from(self.day) - 1) * 86400;
        seconds += ELAPSED_DAYS_MONTHS[self.month as usize - 1] as u64 * 86400;
        seconds += (u64::from(self.year) - 1970) * 365 * 86400;

        // Take into account leap years since 1970.
        seconds += (u64::from(self.year) - 1968) / 4 * 86400;

        // If the current year is a leap year and the current month is
        // January or February, we need to remove one day from the total
        // number of seconds.
        if self.year % 4 == 0 && self.month <= 2 {
            seconds -= 86400;
        }

        Unix(seconds)
    }
}

impl From<Unix> for Date {
    fn from(unix: Unix) -> Self {
        unix_time_to_date(unix)
    }
}

/// # Safety
/// The caller must ensure that the function is only called once, and only
/// during the kernel initialization.
#[init]
pub unsafe fn setup() {
    STARTUP_DATE.write(Date {
        year: arch::hal::date::years(),
        month: arch::hal::date::months(),
        day: arch::hal::date::days(),
        hour: arch::hal::date::hours(),
        minute: arch::hal::date::minutes(),
        second: arch::hal::date::seconds(),
    });
    STARTUP_TIME.write(STARTUP_DATE.read().to_unix_time());

    log::info!("Time: Startup date: {:?}", STARTUP_DATE.read());
    log::info!("Time: Unix time: {:?}", STARTUP_TIME.read());
}

/// Returns the number of days in the given month of the given year. The year
/// is needed to determine if the month of February has 28 or 29 days.
///
/// # Panics
/// This function will panic if the month is not in the range 1..=12.
#[must_use]
pub const fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if Date::leap_year(year as u64) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    }
}

/// Return the number of days elapsed from the beginning of the year to the
/// given month. The month must be in the range 1..=12.
#[must_use]
pub fn month_elsapsed_days(year: u16, month: u8) -> u16 {
    let mut days = 0;
    for m in 1..month {
        days += days_in_month(year, m) as u16;
    }
    days
}

/// Compute the month in the year from the number of days elapsed since the
/// beginning of the year. The year is needed to determine if the month of
/// February has 28 or 29 days. The days must be in the range 1..365.
#[must_use]
pub const fn month_in_year(year: u16, days: u16) -> u8 {
    let mut month_days = 0;
    let mut month = 1;
    while days >= month_days {
        month_days += days_in_month(year, month) as u16;
        month += 1;
    }
    month - 1
}

/// Converts a Unix time to a date.
#[must_use]
pub fn unix_time_to_date(unix: Unix) -> Date {
    let seconds = unix.0;

    // Compute the number of days since January 1st, 1970 and Compute
    // the year with the number of days without taking into account the
    // leap years.
    let unix_days = unix.0 / 86400;
    let year = 1970 + (unix_days / 365);

    // Compute the number of leap years since 1970. Then we compute the days
    // in the current year by removing the number of days since 1970
    let leap_years = (year - 1968) / 4;
    let mut year_days = unix_days;
    year_days -= (year - 1970) * 365;
    year_days -= leap_years;

    // If the current year is a leap year and we are past the 28th of February,
    // we need to add one day to the total number of days.
    if Date::leap_year(year) && year_days >= 59 {
        year_days += 1;
    }

    // Compute the current month of the year, and the day of the month
    let month = month_in_year(year as u16, year_days as u16);
    let day =
        year_days - u64::from(month_elsapsed_days(year as u16, month)) + 1;

    // Compute the hours, minutes and seconds.
    let hour = (seconds / 3600) % 24;
    let minute = (seconds / 60) % 60;
    let second = seconds % 60;

    Date {
        year: year as u16,
        month,
        day: day as u8,
        hour: hour as u8,
        minute: minute as u8,
        second: second as u8,
    }
}
