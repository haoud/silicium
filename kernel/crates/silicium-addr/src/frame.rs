use crate::phys::Physical;
use config::PAGE_SHIFT;

/// A frame is a physical address that represents a page in memory. This is a newtype
/// around `Physical` that represents a frame in the `x86_64` architecture, and enforce
/// that the address is page-aligned. On the `x86_64` architecture, the default page size
/// is 4096 bytes, so a frame address must be a multiple of 4096.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Frame(pub(crate) Physical);

impl Frame {
    /// The maximum frame index supported by the `x86_64` architecture. This is because
    /// physical addresses are 52 bits wide in the `x86_64` architecture.
    pub const MAX_INDEX: usize = Self::MAX.0 .0 >> PAGE_SHIFT;

    /// The maximum frame address supported by the `x86_64` architecture. This is because
    /// physical addresses are 52 bits wide in the `x86_64` architecture.
    pub const MAX: Frame = Frame(Physical(0x000F_FFFF_FFFF_0000));

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

    /// Creates a new `Frame` from the given frame index. This desugars to shifting the
    /// index to the left by the page shift value, which is 12 on the `x86_64` architecture.
    ///
    /// # Panics
    /// Panics if the given index is larger than the maximum supported frame index on the
    /// `x86_64` architecture, which is represented by [`Frame::MAX_INDEX`].
    #[must_use]
    pub const fn from_index(index: usize) -> Self {
        Self::new(index << PAGE_SHIFT)
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

    /// Create a new `Frame` from the given pointer without checking if the pointer is
    /// valid and page-aligned.
    ///
    /// # Safety
    /// The caller must ensure that the given pointer is valid and page-aligned. Otherwise,
    /// the behavior is undefined.
    #[must_use]
    pub unsafe fn from_ptr_unchecked<T>(ptr: *const T) -> Self {
        Self::new_unchecked(ptr as usize)
    }

    /// Return the index associated with the frame. This is the frame number,
    /// which starts at 0 from frame located at address 0x0 and increments by 1
    /// for each frame.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.0 .0 >> PAGE_SHIFT as usize
    }
}

impl const From<Frame> for usize {
    fn from(frame: Frame) -> usize {
        frame.0 .0
    }
}

impl const From<Frame> for u64 {
    fn from(frame: Frame) -> u64 {
        frame.0 .0 as u64
    }
}

impl const core::fmt::Binary for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}

impl const core::fmt::Octal for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#o}", self.0)
    }
}

impl const core::fmt::LowerHex for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl const core::fmt::UpperHex for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

impl const core::fmt::Pointer for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl const core::fmt::Debug for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Virtual({:#x})", self.0)
    }
}

impl const core::fmt::Display for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
