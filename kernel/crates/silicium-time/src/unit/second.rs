use crate::unit::{Millisecond, Millisecond32, Nanosecond, Nanosecond32};
use core::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents a duration in seconds
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Second(pub u64);

impl Second {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl From<Millisecond> for Second {
    fn from(milli: Millisecond) -> Self {
        Self(milli.0 / 1_000)
    }
}

impl From<Nanosecond> for Second {
    fn from(nano: Nanosecond) -> Self {
        Self(nano.0 / 1_000_000_000)
    }
}

impl Add<Second> for Second {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Second> for Second {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub<Second> for Second {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign<Second> for Second {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

/// A 32-bit representation of a duration in seconds
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Second32(pub u32);

impl Second32 {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

impl From<Millisecond32> for Second32 {
    fn from(milli: Millisecond32) -> Self {
        Self(milli.0 / 1_000)
    }
}

impl From<Nanosecond32> for Second32 {
    fn from(nano: Nanosecond32) -> Self {
        Self::new(nano.0 / 1_000_000_000)
    }
}

impl Add<Second32> for Second32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Second32> for Second32 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub<Second32> for Second32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign<Second32> for Second32 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
