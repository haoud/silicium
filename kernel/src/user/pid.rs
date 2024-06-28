use config::MAX_PIDS;
use spin::Spinlock;

/// The size of the bitmap used to keep track of process identifiers.
const PID_BITMAP_COUNT: usize =
    MAX_PIDS as usize / core::mem::size_of::<usize>();

/// A bitmap used to keep track of process identifiers.
static PID_ALLOCATOR: Spinlock<id::Generator<PID_BITMAP_COUNT>> =
    Spinlock::new(id::Generator::new());

/// A process identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pid(u32);

impl Pid {
    /// Generates a new process identifier, unique within the system. If
    /// the maximum number of process identifiers has been reached, returns
    /// `None`.
    #[must_use]
    pub fn generate() -> Option<Self> {
        PID_ALLOCATOR.lock().generate().map(Self)
    }

    /// Deallocates the process identifier, allowing it to be reused.
    pub fn deallocate(self) {
        PID_ALLOCATOR.lock().release(self.0);
    }

    /// Creates a new process identifier with the given `id`
    ///
    /// # Panics
    /// Panics if the `id` is greater than or equal to `MAX_PIDS`.
    #[must_use]
    pub fn new(id: u32) -> Self {
        assert!(id < MAX_PIDS);
        Self(id)
    }

    /// Returns the process identifier as a `u32`.
    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}
