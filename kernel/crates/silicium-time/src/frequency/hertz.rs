/// A representation of a frequency in Hertz
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hertz(pub u64);

impl Hertz {
    /// Create a new `Hertz` instance
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl From<u32> for Hertz {
    fn from(value: u32) -> Self {
        Self::new(u64::from(value))
    }
}

impl From<Hertz32> for Hertz {
    fn from(hz: Hertz32) -> Self {
        Self::new(u64::from(hz.0))
    }
}

/// A 32-bit representation of a frequency in Hertz
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hertz32(pub u32);

impl Hertz32 {
    /// Create a new `Hertz32` instance
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}
