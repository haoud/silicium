#![cfg_attr(not(test), no_std)]
#![allow(clippy::module_name_repetitions)]

use core::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use unit::{Nanosecond, Nanosecond32, Second, Second32};

pub mod frequency;
pub mod unit;

/// A timestamp that represents a point in time with a 32-bit resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timespec32 {
    pub seconds: Second32,
    pub nano: Nanosecond32,
}

impl Timespec32 {
    /// Create a new 32-bits timestamp with the given seconds and nanoseconds. It does
    /// not modify the values to have a "conformant" timestamp (i.e. the nanoseconds
    /// are less than `1_000_000_000`).
    #[must_use]
    pub const fn new(seconds: Second32, nano: Nanosecond32) -> Self {
        Self { seconds, nano }
    }

    /// Create a new conformant 32-bits timestamp with the given seconds and
    /// nanoseconds. It nanoseconds are greater than `1_000_000_000`, it will
    /// increment the seconds accordingly and adjust the nanoseconds.
    #[must_use]
    pub fn conform(second: Second32, nano: Nanosecond32) -> Self {
        let mut seconds = second;
        let mut nano = nano;

        seconds.0 += nano.0 / 1_000_000_000;
        nano.0 %= 1_000_000_000;
        Self { seconds, nano }
    }
}

/// A timestamp that represents a point in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timespec {
    pub seconds: Second,
    pub nano: Nanosecond,
}

impl Timespec {
    /// Create a new timestamp with the given seconds and nanoseconds. It does
    /// not modify the values to have a "conformant" timestamp (i.e. the nanoseconds
    /// are less than `1_000_000_000`).
    #[must_use]
    pub const fn new(seconds: Second, nano: Nanosecond) -> Self {
        Self { seconds, nano }
    }

    /// Create a new conformant timestamp with the given seconds and
    /// nanoseconds. It nanoseconds are greater than `1_000_000_000`, it will
    /// increment the seconds accordingly and adjust the nanoseconds.
    #[must_use]
    pub fn conform(second: Second, nano: Nanosecond) -> Self {
        let mut seconds = second;
        let mut nano = nano;

        seconds.0 += nano.0 / 1_000_000_000;
        nano.0 %= 1_000_000_000;
        Self { seconds, nano }
    }
}

impl Add<Second> for Timespec {
    type Output = Self;

    fn add(self, rhs: Second) -> Self::Output {
        Self {
            seconds: self.seconds + rhs,
            nano: self.nano,
        }
    }
}

impl Add<Nanosecond> for Timespec {
    type Output = Self;

    fn add(self, rhs: Nanosecond) -> Self::Output {
        Self::conform(self.seconds, self.nano + rhs)
    }
}

impl AddAssign<Second> for Timespec {
    fn add_assign(&mut self, rhs: Second) {
        *self = *self + rhs;
    }
}

impl AddAssign<Nanosecond> for Timespec {
    fn add_assign(&mut self, rhs: Nanosecond) {
        *self = *self + rhs;
    }
}

impl Sub<Second> for Timespec {
    type Output = Self;

    fn sub(self, rhs: Second) -> Self::Output {
        Self {
            seconds: self.seconds - rhs,
            nano: self.nano,
        }
    }
}

impl Sub<Nanosecond> for Timespec {
    type Output = Self;

    fn sub(self, mut rhs: Nanosecond) -> Self::Output {
        let mut seconds = self.seconds;
        while rhs >= Nanosecond::new(1_000_000_000) {
            rhs += Nanosecond::new(1_000_000_000);
            seconds -= Second::new(1);
        }

        Self::conform(seconds, rhs)
    }
}

impl SubAssign<Second> for Timespec {
    fn sub_assign(&mut self, rhs: Second) {
        *self = *self - rhs;
    }
}

impl SubAssign<Nanosecond> for Timespec {
    fn sub_assign(&mut self, rhs: Nanosecond) {
        *self = *self - rhs;
    }
}

impl Display for Timespec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{} s", self.seconds.0, self.nano.0)
    }
}
