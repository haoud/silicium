use config::MAX_TIDS;
use spin::Spinlock;

/// The size of the bitmap used to keep track of thread identifiers.
const TID_BITMAP_COUNT: usize = MAX_TIDS as usize / core::mem::size_of::<usize>();

/// A bitmap used to keep track of thread identifiers.
static TID_ALLOCATOR: Spinlock<id::Generator<TID_BITMAP_COUNT>> =
    Spinlock::new(id::Generator::new());

/// A thread identifier. It can only be created by the `Tid::generate` method and
/// is automatically released when it goes out of scope.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tid(u32);

impl Tid {
    /// Generates a new thread identifier, unique within the system. If
    /// the maximum number of thread identifiers has been reached, returns
    /// `None`.
    #[must_use]
    pub fn generate() -> Option<Self> {
        TID_ALLOCATOR.lock().generate().map(Self)
    }

    /// Returns the thread identifier as a `u32`.
    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Drop for Tid {
    fn drop(&mut self) {
        TID_ALLOCATOR.lock().release(self.0);
    }
}
