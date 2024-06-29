use crate::{arch, time};
use core::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

/// A point in time, relative to the Unix epoch. It is guaranteed to behave as
/// a monotonically non-decreasing clock and is guaranteed to be not less than
/// any previously `Instant` value created. This is especially useful for
/// tasks such as measuring benchmarks or timing how long an operation takes.
///
/// Note, however, that instants are not guaranteed to be steady. In other
/// words, each tick of the underlying clock might not be the same length
/// (e.g. some seconds may be longer than others). An instant may jump forwards
/// or experience time dilation (slow down or speed up), but it will never go
/// backwards.
///
/// # Panics
/// Operations such as `Add` and AddAssign` will panic if the resulting
/// instant would overflow. However, this only happens after more than
/// 292 billion years of time since the Unix epoch, and is probably a bug
/// in the program.
/// However, `Sub` and `SubAssign` will panic if the resulting instant would
/// be negative (i.e. if the right-hand side instant is later than the
/// left-hand)
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
