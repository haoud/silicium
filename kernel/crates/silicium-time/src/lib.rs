#![cfg_attr(not(test), no_std)]
#![allow(clippy::module_name_repetitions)]

use core::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use unit::{Nanosecond, Second};

pub mod frequency;
pub mod unit;

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

    /// Create a new timestamp with all fields set to zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            seconds: Second(0),
            nano: Nanosecond(0),
        }
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

    fn normalize(&mut self) {
        self.seconds.0 += self.nano.0 / 1_000_000_000;
        self.nano.0 %= 1_000_000_000;
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

impl Add<Timespec> for Timespec {
    type Output = Self;

    fn add(self, rhs: Timespec) -> Self::Output {
        let mut copy = self;
        copy += rhs.seconds;
        copy += rhs.nano;
        self
    }
}

impl AddAssign<Second> for Timespec {
    fn add_assign(&mut self, rhs: Second) {
        self.seconds += rhs;
    }
}

impl AddAssign<Nanosecond> for Timespec {
    fn add_assign(&mut self, rhs: Nanosecond) {
        self.nano += rhs;
        self.normalize();
    }
}

impl AddAssign<Timespec> for Timespec {
    fn add_assign(&mut self, rhs: Timespec) {
        self.seconds += rhs.seconds;
        self.nano += rhs.nano;
        self.normalize();
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

    fn sub(self, rhs: Nanosecond) -> Self::Output {
        if rhs.0 > self.nano.0 {
            let mut seconds = self.seconds;
            let mut nano = self.nano;

            let diff = rhs.0 - nano.0;
            nano.0 = 1_000_000_000 - diff;
            seconds -= Second(rhs.0 / 1_000_000_000 + 1);
            Self::conform(seconds, nano)
        } else {
            Self::conform(self.seconds, self.nano - rhs)
        }
    }
}

impl Sub<Timespec> for Timespec {
    type Output = Self;

    fn sub(self, rhs: Timespec) -> Self::Output {
        let mut copy = self;
        copy -= rhs.seconds;
        copy -= rhs.nano;
        copy
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

impl SubAssign<Timespec> for Timespec {
    fn sub_assign(&mut self, rhs: Timespec) {
        *self = *self - rhs;
    }
}

impl Display for Timespec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{:09} s", self.seconds.0, self.nano.0)
    }
}
