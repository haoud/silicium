pub mod millis;
pub mod nano;
pub mod second;

pub use millis::{Millisecond, Millisecond32};
pub use nano::{Nanosecond, Nanosecond32};
pub use second::{Second, Second32};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Overflow(());
