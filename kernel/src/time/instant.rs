use crate::{arch, time};
use core::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

/// TODO:
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(Duration);

impl Instant {
    /// Returns an instant corresponding to the Unix epoch. This is the
    /// earliest instant representable by the `Instant` type.
    pub const EPOCH: Self = Self(Duration::from_secs(0));

    /// Returns an instant corresponding to "now".
    #[must_use]
    pub fn now() -> Self {
        let time_boot = Duration::from_secs(u64::from(time::Unix::boot()));
        let since_boot = arch::time::since_boot();
        Self(time_boot + since_boot)
    }
}

impl Sub for Instant {
    type Output = Duration;

    /// Returns the amount of time elapsed from the other instant to this one,
    /// or zero if that instant is later than this one.
    fn sub(self, rhs: Self) -> Self::Output {
        self.0.saturating_sub(rhs.0)
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    /// Add a duration to this instant, returning a new instant.
    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    /// Subtract a duration from this instant, returning a new instant.
    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl AddAssign<Duration> for Instant {
    /// Add a duration to this instant.
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs;
    }
}

impl SubAssign<Duration> for Instant {
    /// Subtract a duration from this instant.
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs;
    }
}
