use super::Frame;
use core::{
    iter::Step,
    ops::{Add, AddAssign, Sub, SubAssign},
};

/// A physical address is an address that points to a location in the physical
/// memory of the system. The physical memory is the actual RAM that is used
/// by the system to store data. When paging is enabled, the physical memory
/// is not directly accessible by the CPU. Instead, the CPU uses virtual
/// addresses, which are translated to physical addresses by the memory
/// management unit (MMU). This allows the operating system to provide each
/// process with its own virtual address space, while still sharing the
/// physical memory between processes.
///
/// This type enforces that the address is 52 bits wide or less, which is the
/// maximum supported physical address on the `x86_64` architecture.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Physical(usize);

impl Physical {
    /// The last valid physical address on the `x86_64` architecture.
    pub const MAX: Physical = Physical(0x000F_FFFF_FFFF_FFFF);

    /// The zero physical address.
    pub const ZERO: Physical = Physical(0);

    /// Creates a new `Physical` address
    ///
    /// # Panics
    /// Panics if the given address is larger than the maximum supported
    /// physical address on the `x86_64` architecture, which is represented
    /// by [`Physical::MAX`].
    #[must_use]
    pub const fn new(addr: usize) -> Self {
        Self::try_new(addr)
            .expect("Physical address must be less than 52 bits wide")
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
    /// is represented by [`Physical::MAX`].
    #[must_use]
    pub const unsafe fn new_unchecked(addr: usize) -> Self {
        Physical(addr)
    }

    /// Creates a new `Physical` address by truncating the given address to an
    /// 52-bit address.
    #[must_use]
    pub const fn new_truncate(addr: usize) -> Self {
        Self::new(addr & Self::MAX.0)
    }

    /// Align down the physical address to the given alignment. If the physical
    /// address is already aligned to the given alignment, the address will not
    /// be changed.
    ///
    /// # Panics
    /// Panic if the alignement is not a power of two
    #[must_use]
    pub const fn align_down(self, align: usize) -> Self {
        assert!(align.is_power_of_two());
        Self(self.0 & !(align - 1))
    }

    /// Align up the physical address to the given alignment. The alignment
    /// must be a power of two, otherwise the result will be incorrect. If the
    /// physical address is already aligned to the given alignment, the address
    /// will not be changed.
    #[must_use]
    pub const fn align_up(self, align: usize) -> Self {
        assert!(align.is_power_of_two());
        Self((self.0 + align - 1) & !(align - 1))
    }

    /// Verify if the physical address is aligned to the given alignment.
    ///
    /// # Panics
    /// Panic if the alignement is not a power of two.
    #[must_use]
    pub const fn is_aligned_to(self, align: usize) -> bool {
        assert!(align.is_power_of_two());
        self.0 & (align - 1) == 0
    }

    /// Returns the address as a mutable pointer to the specified type.
    #[must_use]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }

    /// Returns the address as a const pointer to the specified type.
    #[must_use]
    pub const fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    /// Returns the address as a `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Returns the address as a `u64`.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0 as u64
    }

    /// Checks if the address is zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
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

impl Add<u64> for Physical {
    type Output = Self;

    /// Add an offset to the physical address.
    ///
    /// # Panics
    /// Panics if the resulting address is larger than the maximum supported
    /// physical address on the `x86_64` architecture, which is represented by
    /// [`Physical::MAX`].
    fn add(self, rhs: u64) -> Self {
        Self::new(self.0 + rhs as usize)
    }
}

impl Add<usize> for Physical {
    type Output = Self;

    /// Add an offset to the physical address.
    ///
    /// # Panics
    /// Panics if the resulting address is larger than the maximum supported
    /// physical address on the `x86_64` architecture, which is represented by
    /// [`Physical::MAX`].
    fn add(self, rhs: usize) -> Self {
        Self::new(self.0 + rhs)
    }
}

impl Sub<u64> for Physical {
    type Output = Self;

    /// Subtract an offset from the physical address.
    ///
    /// # Panics
    /// Panics if the resulting address would underflow.
    fn sub(self, rhs: u64) -> Self {
        Self::new(self.0 - rhs as usize)
    }
}

impl Sub<usize> for Physical {
    type Output = Self;

    /// Subtract an offset from the physical address.
    ///
    /// # Panics
    /// Panics if the resulting address would underflow.
    fn sub(self, rhs: usize) -> Self {
        Self::new(self.0 - rhs)
    }
}

impl AddAssign<u64> for Physical {
    /// Add an offset to the physical address.
    ///
    /// # Panics
    /// Panics if the resulting address is larger than the maximum supported7
    /// physical address on the `x86_64` architecture, which is represented by
    /// [`Physical::MAX`].
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

impl AddAssign<usize> for Physical {
    /// Add an offset to the physical address.
    ///
    /// # Panics
    /// Panics if the resulting address is larger than the maximum supported
    /// physical address on the `x86_64` architecture, which is represented by
    /// [`Physical::MAX`].
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl SubAssign<u64> for Physical {
    /// Subtract an offset from the physical address.
    ///
    /// # Panics
    /// Panics if the resulting address would underflow.
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

impl SubAssign<usize> for Physical {
    /// Subtract an offset from the physical address.
    ///
    /// # Panics
    /// Panics if the resulting address would underflow.
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs;
    }
}

impl Step for Physical {
    /// The number of steps between two physical addresses is simply
    /// the difference between the two addresses value !
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if end.0 >= start.0 {
            Some(end.0 - start.0)
        } else {
            None
        }
    }

    /// Advances the physical address by `count` bytes. If the address
    /// overflows or is greater than the maximum physical address, then
    /// `None` is returned.
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Self::try_new(start.0.checked_add(count)?)
    }

    /// Retreats the physical address by `count` bytes. If the address
    /// underflows, then `None` is returned.
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Self::try_new(start.0.checked_sub(count)?)
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
