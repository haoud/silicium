use core::{iter::Step, marker::PhantomData};

/// The type of a virtual address. It can either be a user-space address or a
/// kernel-space address.
pub trait Type: Copy {}

/// A user-space virtual address marker.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct User;
impl Type for User {}

/// A kernel-space virtual address marker.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kernel;
impl Type for Kernel {}

/// A virtual address is an address that points to a location in the virtual
/// memory of the system. The virtual memory is an abstraction that provides
/// each process with its own address space, which is isolated from the address
/// spaces of other processes. The address spaces are divided into two regions:
/// the user-space and the kernel-space. The user-space is used to store the
/// code and data of the process, while the kernel-space is used to store the
/// code and data of the kernel. The kernel-space is shared between all
/// processes, while the user-space is private to each process.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Virtual<T>(usize, PhantomData<T>);

impl<T: Type> Virtual<T> {
    /// Create a new virtual address without performing any checks.
    ///
    /// # Safety
    /// The caller must ensure that the virtual address is valid according to
    /// the requested variant (`KERNEL` or `USER`)
    #[must_use]
    pub const unsafe fn new_unchecked(addr: usize) -> Self {
        Self(addr, PhantomData)
    }

    /// Create a new virtual address from a pointer without performing any
    /// checks.
    ///
    /// # Safety
    /// The caller must ensure that the virtual address is valid according to
    /// the requested variant (`KERNEL` or `USER`)
    #[must_use]
    pub unsafe fn from_ptr_unchecked<P>(ptr: *const P) -> Self {
        Self::new_unchecked(ptr as usize)
    }

    /// Return the physical address as a mutable pointer.
    #[must_use]
    pub const fn as_mut_ptr<P>(&self) -> *mut P {
        self.0 as *mut P
    }

    /// Return the physical address as a const pointer.
    #[must_use]
    pub const fn as_ptr<P>(&self) -> *const P {
        self.0 as *const P
    }

    /// Return the address as a `usize`.
    #[must_use]
    pub const fn as_usize(&self) -> usize {
        self.0
    }

    /// Return the address as a `u64`.
    #[must_use]
    pub const fn as_u64(&self) -> u64 {
        self.0 as u64
    }

    /// Check if the address is zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Virtual<User> {
    /// The minimum valid user virtual address, assuming a 39-bit virtual
    /// address space.
    pub const START: Self = Self(0x0000_0000_0000_0000, PhantomData);

    /// The maximum valid user virtual address, assuming a 39-bit virtual
    /// address space.
    pub const END: Self = Self(0x0000_7FFF_FFFF_FFFF, PhantomData);

    /// Create a new user virtual address.
    ///
    /// # Panics
    /// This function will panic if the address is not in the user
    /// address space (as defined by [`START`] and [`END`]).
    #[must_use]
    pub const fn new(addr: usize) -> Self {
        match Self::try_new(addr) {
            None => panic!("User virtual address out of bounds"),
            Some(v) => v,
        }
    }

    /// Attempt to create a new user virtual address. If the address is not
    /// in the user address space (as defined by [`START`] and [`END`]), then
    /// `None` is returned.
    #[must_use]
    pub const fn try_new(addr: usize) -> Option<Self> {
        if addr <= Self::END.0 {
            Some(Self(addr, PhantomData))
        } else {
            None
        }
    }
}

impl Step for Virtual<User> {
    /// The number of steps between two user virtual addresses is simply the
    /// difference between the two addresses.
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if end >= start {
            Some(end.0 - start.0)
        } else {
            None
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Self::try_new(start.0 + count)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Self::try_new(start.0 - count)
    }
}

impl Virtual<Kernel> {
    /// The minimum valid kernel virtual address, assuming a 39-bit virtual
    /// address space.
    pub const START: Self = Self(0xFFFF_8000_0000_0000, PhantomData);

    /// The maximum valid kernel virtual address, assuming a 39-bit virtual
    /// address space.
    pub const END: Self = Self(0xFFFF_FFFF_FFFF_FFFF, PhantomData);

    /// Create a new kernel virtual address.
    ///
    /// # Panics
    /// This function will panic if the address is not in the kernel
    /// address space (as defined by [`START`] and [`END`]).
    #[must_use]
    pub const fn new(addr: usize) -> Self {
        match Self::try_new(addr) {
            None => panic!("Kernel virtual address out of bounds"),
            Some(v) => v,
        }
    }

    /// Attempt to create a new kernel virtual address. If the address is not
    /// in the kernel address space (as defined by [`START`] and [`END`]),
    /// then `None` is returned.
    #[must_use]
    pub const fn try_new(addr: usize) -> Option<Self> {
        if addr >= Self::START.0 {
            Some(Self(addr, PhantomData))
        } else {
            None
        }
    }

    /// Create a new kernel virtual address from a pointer.
    ///
    /// # Panics
    /// This function will panic if the address is not in the kernel
    /// address space (as defined by [`START`] and [`END`]).
    #[must_use]
    pub fn from_ptr<P>(ptr: *const P) -> Self {
        Self::new(ptr as usize)
    }

    /// Align the address up to the nearest page boundary. If the address is
    /// already page aligned, then it is returned as is.
    ///
    /// # Panics
    /// This function will panic if the resulting address cannot fit into an
    /// `u64` (the address is greater than [`MAX`]).
    #[must_use]
    pub const fn page_align_up(&self) -> Self {
        Self::new((self.0 + 4096 - 1) & !(4096 - 1))
    }
}

impl<T: Type> From<Virtual<T>> for usize {
    fn from(addr: Virtual<T>) -> Self {
        addr.as_usize()
    }
}

impl<T: Type> From<Virtual<T>> for u64 {
    fn from(addr: Virtual<T>) -> Self {
        addr.as_u64()
    }
}

impl Step for Virtual<Kernel> {
    /// The number of steps between two kernel virtual addresses is
    /// simply the difference between the two addresses.
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if end >= start {
            Some(end.0 - start.0)
        } else {
            None
        }
    }

    /// Advances the virtual address by `count` bytes. If the resulting
    /// address is not in the kernel address space or overflows, then
    /// `None` is returned.
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Self::try_new(start.0.checked_add(count)?)
    }

    /// Retreats the virtual address by `count` bytes. If the resulting
    /// address is not in the kernel address space or underflows, then
    /// `None` is returned.
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Self::try_new(start.0.checked_sub(count)?)
    }
}

impl<T: Type> core::fmt::Binary for Virtual<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}

impl<T: Type> core::fmt::Octal for Virtual<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#o}", self.0)
    }
}

impl<T: Type> core::fmt::LowerHex for Virtual<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl<T: Type> core::fmt::UpperHex for Virtual<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

impl<T: Type> core::fmt::Pointer for Virtual<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl<T: Type> core::fmt::Debug for Virtual<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Virtual({:#x})", self.0)
    }
}

impl<T: Type> core::fmt::Display for Virtual<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
