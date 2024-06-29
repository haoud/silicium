#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Virtual(pub(crate) usize);

impl Virtual {
    /// Creates a new `Virtual` address
    ///
    /// # Panics
    /// Panics if the given address is not canonical, which means that the
    /// most significant 16 bits of the address must be either all ones or
    /// all zeros. Put more simply, the address can't be between
    /// `0x0000_8000_0000_0000` and `0xFFFF_7FFF_FFFF_FFFF` (inclusive).
    #[must_use]
    pub const fn new(addr: usize) -> Self {
        match Self::try_new(addr) {
            None => panic!("Virtual address is not canonical"),
            Some(addr) => addr,
        }
    }

    /// Creates a new `Virtual` address if the given address is canonical.
    /// Returns `None` if the address is not canonical.
    ///
    /// For an definition of what is a canonical address, see the
    /// documentation for the [`new`] method.
    #[must_use]
    pub const fn try_new(addr: usize) -> Option<Self> {
        match (addr & 0xFFFF_8000_0000_0000) >> 47 {
            0 | 0x1FFFF => Some(Self(addr)),
            _ => None,
        }
    }

    /// Create a new `Virtual` address without checking if the address is
    /// canonical.
    ///
    /// # Safety
    /// The caller must ensure that the given address is canonical. For an
    /// definition of what is a canonical address, see the documentation for
    /// the [`new`] method.
    #[must_use]
    pub const unsafe fn new_unchecked(addr: usize) -> Self {
        Self(addr)
    }

    /// Creates a new `Virtual` address from a pointer.
    ///
    /// # Panics
    /// Panics if the given pointer is not canonical. For an definition of
    /// what is a canonical address, see the documentation for the [`new`]
    /// method.
    #[must_use]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::new(ptr as usize)
    }

    /// Creates a new `Virtual` address from a pointer without checking if
    /// the address is canonical.
    ///
    /// # Safety
    /// The caller must ensure that the given pointer is canonical. For an
    /// definition of what is a canonical address, see the documentation for
    /// the [`new`] method. If the provided pointer point to a valid object,
    /// then the address should be canonical. In practice, this method should
    /// be safe to use in most cases.
    #[must_use]
    pub unsafe fn from_ptr_unchecked<T>(ptr: *const T) -> Self {
        Self::new_unchecked(ptr as usize)
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

    /// If the address is page-aligned, returns `true`. Otherwise, returns
    /// `false`. For reference, the default page size on the `x86_64`
    /// architecture is 4096 bytes.
    #[must_use]
    pub const fn is_page_aligned(&self) -> bool {
        (self.0 & 0xFFF) == 0
    }

    /// Align the address to the previous page aligned address. If the
    /// address is already page-aligned, the same address is returned.
    #[must_use]
    pub const fn page_align_down(&self) -> Self {
        Self(self.0 & !0xFFF)
    }

    /// Align the address to the next page aligned address. If the address
    /// is already page-aligned, the same address is returned.
    #[must_use]
    pub const fn page_align_up(&self) -> Self {
        Self((self.0 + 0xFFF) & !0xFFF)
    }
}

impl From<Virtual> for usize {
    fn from(addr: Virtual) -> usize {
        addr.0
    }
}

impl From<Virtual> for u64 {
    fn from(addr: Virtual) -> u64 {
        addr.0 as u64
    }
}

impl core::fmt::Binary for Virtual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}

impl core::fmt::Octal for Virtual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#o}", self.0)
    }
}

impl core::fmt::LowerHex for Virtual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl core::fmt::UpperHex for Virtual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

impl core::fmt::Pointer for Virtual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl core::fmt::Debug for Virtual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Virtual({:#x})", self.0)
    }
}

impl core::fmt::Display for Virtual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
