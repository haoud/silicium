use crate::unit::{Nanosecond, Nanosecond32, Overflow, Second, Second32};

/// Represents a duration in milliseconds
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Millisecond(pub u64);

impl Millisecond {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl From<Second> for Millisecond {
    fn from(second: Second) -> Self {
        Self(second.0 * 1_000)
    }
}

impl TryFrom<Nanosecond> for Millisecond {
    type Error = Overflow;

    fn try_from(ns: Nanosecond) -> Result<Self, Self::Error> {
        ns.0.checked_mul(1_000_000)
            .map(Self::new)
            .ok_or(Overflow(()))
    }
}

/// A 32-bit representation of a duration in milliseconds
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Millisecond32(pub u32);

impl Millisecond32 {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

impl From<Second32> for Millisecond32 {
    fn from(second: Second32) -> Self {
        Self(second.0 * 1_000)
    }
}

impl TryFrom<Nanosecond32> for Millisecond32 {
    type Error = Overflow;

    fn try_from(ns: Nanosecond32) -> Result<Self, Self::Error> {
        ns.0.checked_mul(1_000_000)
            .map(Self::new)
            .ok_or(Overflow(()))
    }
}
