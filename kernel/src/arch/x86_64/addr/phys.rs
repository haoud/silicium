use super::Frame;

/// A physical address. This is a newtype around `usize` that represents a
/// physical address in the `x86_64` architecture that enforces the maximum
/// physical address supported by the architecture (52 bits wide, represented
/// by [`Physical::MAX`]).
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Physical(pub(crate) usize);

impl Physical {
    /// The maximum physical address supported by the `x86_64` architecture.
    /// This is because physical addresses are 52 bits wide in the `x86_64`
    /// architecture.
    pub const MAX: Physical = Physical(0x000F_FFFF_FFFF_FFFF);

    /// Creates a new `Physical` address
    ///
    /// # Panics
    /// Panics if the given address is larger than the maximum supported
    /// physical address on the `x86_64` architecture, which is represented
    /// by [`Physical::MAX`].
    #[must_use]
    pub const fn new(addr: usize) -> Self {
        match Self::try_new(addr) {
            None => panic!(
                "Physical address is not valid (must be 52 bits wide or less)"
            ),
            Some(addr) => addr,
        }
    }

    /// Creates a new `Physical` address if the given address is valid.
    /// Otherwise, returns `None`.
    #[must_use]
    pub const fn try_new(addr: usize) -> Option<Self> {
        if addr <= Self::MAX.0 {
            Some(Physical(addr))
        } else {
            None
        }
    }

    /// Create a new `Physical` address without checking if the address is
    /// valid.
    ///
    /// # Safety
    /// The caller must ensure that the given address is less or equal to the
    /// maximum supported physical address on the `x86_64` architecture, which
    /// is represented by [`Physical::MAX`]. Otherwise, the behavior is
    /// undefined.
    #[must_use]
    pub const unsafe fn new_unchecked(addr: usize) -> Self {
        Physical(addr)
    }

    /// If the address is page-aligned, returns `true`. Otherwise, returns
    /// `false`. For reference, the default page size on the `x86_64`
    /// architecture is 4096 bytes.
    #[must_use]
    pub const fn is_page_aligned(&self) -> bool {
        (self.0 & 0xFFF) == 0
    }

    /// Returns the address as a mutable pointer to the specified type.
    #[must_use]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }

    /// Returns the address as a pointer to the specified type.
    #[must_use]
    pub const fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }
}

impl From<Physical> for usize {
    fn from(addr: Physical) -> usize {
        addr.0
    }
}

impl From<Physical> for u64 {
    fn from(addr: Physical) -> u64 {
        addr.0 as u64
    }
}

impl From<Frame> for Physical {
    fn from(frame: Frame) -> Physical {
        frame.0
    }
}

impl core::fmt::Binary for Physical {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}

impl core::fmt::Octal for Physical {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#o}", self.0)
    }
}

impl core::fmt::LowerHex for Physical {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl core::fmt::UpperHex for Physical {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

impl core::fmt::Pointer for Physical {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl core::fmt::Debug for Physical {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Virtual({:#x})", self.0)
    }
}

impl core::fmt::Display for Physical {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
