use config::MAX_TIDS;

/// The size of the bitmap used to keep track of thread identifiers.
const TID_BITMAP_COUNT: usize = MAX_TIDS as usize / core::mem::size_of::<usize>();

/// A bitmap used to keep track of thread identifiers.
static TID_ALLOCATOR: spin::Mutex<id::Generator<TID_BITMAP_COUNT>> =
    spin::Mutex::new(id::Generator::new());

/// A thread identifier. It can only be created by the `Tid::generate` method and
/// is automatically released when it goes out of scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tid(u32);

impl Tid {
    /// Generates a new thread identifier, unique within the system. If
    /// the maximum number of thread identifiers has been reached, returns
    /// `None`.
    #[must_use]
    pub fn generate() -> Option<Self> {
        TID_ALLOCATOR.lock().generate().map(Self)
    }

    /// Deallocates the thread identifier, allowing it to be reused.
    pub fn deallocate(self) {
        TID_ALLOCATOR.lock().release(self.0);
    }

    /// Creates a new thread identifier from a `u32`.
    ///
    /// # Panics
    /// Panics if the `id` is greater than or equal to `MAX_TIDS`.
    pub fn new(id: u32) -> Self {
        assert!(id < MAX_TIDS);
        Self(id)
    }

    /// Returns the thread identifier as a `u32`.
    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}
