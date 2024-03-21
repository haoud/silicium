use config::MAX_PIDS;
use spin::Spinlock;

/// The size of the bitmap used to keep track of process identifiers.
const PID_BITMAP_COUNT: usize = MAX_PIDS as usize / core::mem::size_of::<usize>();

/// A bitmap used to keep track of process identifiers.
static PID_ALLOCATOR: Spinlock<id::Generator<PID_BITMAP_COUNT>> =
    Spinlock::new(id::Generator::new());

/// A process identifier. It can only be created by the `Pid::generate` method and
/// is automatically released when it goes out of scope.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pid(u32);

impl Pid {
    /// Generates a new process identifier, unique within the system. If
    /// the maximum number of process identifiers has been reached, returns
    /// `None`.
    #[must_use]
    pub fn generate() -> Option<Self> {
        PID_ALLOCATOR.lock().generate().map(Self)
    }

    /// Returns the process identifier as a `u32`.
    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Drop for Pid {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().release(self.0);
    }
}
