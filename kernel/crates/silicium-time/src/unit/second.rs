use super::{Nanosecond, Nanosecond32};

/// Represents a duration in seconds
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Second(pub u64);

impl Second {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl From<Nanosecond> for Second {
    fn from(nano: Nanosecond) -> Self {
        Self(nano.0 / 1_000_000_000)
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

impl From<Nanosecond32> for Second32 {
    fn from(nano: Nanosecond32) -> Self {
        Self::new(nano.0 / 1_000_000_000)
    }
}
