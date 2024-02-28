pub use crate::phys::Physical;

/// A frame is a physical address that represents a page in memory. This is a newtype
/// around `Physical` that represents a frame in the `x86_64` architecture, and enforce
/// that the address is page-aligned. On the `x86_64` architecture, the default page size
/// is 4096 bytes, so a frame address must be a multiple of 4096.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Frame(pub(crate) Physical);

impl Frame {
    /// Creates a new `Frame` from the given address.
    ///
    /// # Panics
    /// Panics if the given address is not page-aligned or is larger than the maximum
    /// supported physical address on the `x86_64` architecture, which is represented by
    /// [`Physical::MAX`].
    #[must_use]
    pub const fn new(addr: usize) -> Self {
        match Self::try_new(addr) {
            None => {
                panic!("Frame address is not valid (must be 52 bits wide or less and page-aligned)")
            }
            Some(addr) => addr,
        }
    }

    /// Creates a new `Frame` address if the given address is valid and page-aligned.
    /// Otherwise, returns `None`.
    #[must_use]
    pub const fn try_new(addr: usize) -> Option<Self> {
        match Physical::try_new(addr) {
            Some(addr) => {
                if addr.is_page_aligned() {
                    Some(Frame(addr))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Create a new `Frame` address without checking if the address is valid
    /// and page-aligned.
    ///
    /// # Safety
    /// The caller must ensure that the given address is less or equal to the maximum
    /// supported physical address on the `x86_64` architecture, which is represented
    /// by [`Physical::MAX`], and that the address is page-aligned. Otherwise, the
    /// behavior is undefined.
    #[must_use]
    pub const unsafe fn new_unchecked(addr: usize) -> Self {
        Frame(Physical::new_unchecked(addr))
    }

    /// Creates a new `Frame` from the given pointer.
    ///
    /// # Panics
    /// Panics if the given pointer is not page-aligned or is larger than the maximum
    /// supported physical address on the `x86_64` architecture, which is represented by
    /// [`Physical::MAX`].
    #[must_use]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::new(ptr as usize)
    }

    /// Creates a new `Frame` from the given pointer without checking if the pointer is
    /// valid and page-aligned.
    ///
    /// # Safety
    /// The caller must ensure that the given pointer is valid and page-aligned. Otherwise,
    /// the behavior is undefined.
    #[must_use]
    pub unsafe fn from_ptr_unchecked<T>(ptr: *const T) -> Self {
        Self::new_unchecked(ptr as usize)
    }
}

impl From<Frame> for usize {
    fn from(frame: Frame) -> usize {
        frame.0 .0
    }
}

impl From<Frame> for u64 {
    fn from(frame: Frame) -> u64 {
        frame.0 .0 as u64
    }
}
