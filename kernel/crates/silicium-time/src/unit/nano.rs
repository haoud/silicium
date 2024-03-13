use crate::unit::{Millisecond, Millisecond32, Overflow, Second, Second32};

/// Represents a duration in nanoseconds
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nanosecond(pub u64);

impl Nanosecond {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl TryFrom<Second> for Nanosecond {
    type Error = Overflow;

    fn try_from(second: Second) -> Result<Self, Self::Error> {
        second
            .0
            .checked_mul(1_000_000_000)
            .map(Self::new)
            .ok_or(Overflow(()))
    }
}

impl TryFrom<Millisecond> for Nanosecond {
    type Error = Overflow;

    fn try_from(milli: Millisecond) -> Result<Self, Self::Error> {
        milli
            .0
            .checked_mul(1_000_000)
            .map(Self::new)
            .ok_or(Overflow(()))
    }
}

/// A 32-bit representation of a duration in nanoseconds
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nanosecond32(pub u32);

impl Nanosecond32 {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<Second32> for Nanosecond32 {
    type Error = Overflow;

    fn try_from(second: Second32) -> Result<Self, Self::Error> {
        second
            .0
            .checked_mul(1_000_000_000)
            .map(Self::new)
            .ok_or(Overflow(()))
    }
}

impl TryFrom<Millisecond32> for Nanosecond32 {
    type Error = Overflow;

    fn try_from(milli: Millisecond32) -> Result<Self, Self::Error> {
        milli
            .0
            .checked_mul(1_000_000)
            .map(Self::new)
            .ok_or(Overflow(()))
    }
}
