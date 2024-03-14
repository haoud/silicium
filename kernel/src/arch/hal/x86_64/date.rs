use crate::arch::x86_64::cmos::{read, Register};

/// Get the current year from the CMOS. Since it reads the CMOS
/// through I/O ports, it is very slow and should be used sparingly.
///
/// This function does not read the century register and assumes
/// that we are in the 21st century (a relatively safe assumption...)
#[must_use]
pub fn years() -> u16 {
    2000 + u16::from(read(Register::Year))
}

/// Get the current month from the CMOS. Since it reads the CMOS
/// through I/O ports, it is very slow and should be used sparingly.
#[must_use]
pub fn months() -> u8 {
    read(Register::Month)
}

/// Get the current days time from the CMOS. Since it reads the CMOS
/// through I/O ports, it is very slow and should be used sparingly.
#[must_use]
pub fn days() -> u8 {
    read(Register::Day)
}

/// Get the current hours time from the CMOS. Since it reads the CMOS
/// through I/O ports, it is very slow and should be used sparingly.
#[must_use]
pub fn hours() -> u8 {
    read(Register::Hours)
}

/// Get the current minutes time from the CMOS. Since it reads the CMOS
/// through I/O ports, it is very slow and should be used sparingly.
#[must_use]
pub fn minutes() -> u8 {
    read(Register::Minutes)
}

/// Get the current seconds time from the CMOS. Since it reads the CMOS
/// through I/O ports, it is very slow and should be used sparingly.
#[must_use]
pub fn seconds() -> u8 {
    read(Register::Seconds)
}
