use addr::{Frame, Physical, Virtual};
use zerocopy::FromBytes;

/// Obtain an mutable reference to the object located at the given physical address
/// during the execution of the given closure.
///
/// # Safety
/// The caller must ensure that the address is properly aligned to the type `T`, and
/// that the address is not aliased anywhere in the kernel. Failure to do so will
/// result in undefined behavior !
#[inline]
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn access_mut<T, Object: FromBytes>(
    _phys: impl Into<Physical>,
    _f: impl FnOnce(&mut Object) -> T,
) -> T {
    unimplemented!()
}

/// Obtain a reference to the object located at the given physical address during
/// the execution of the given closure.
///
/// # Safety
/// The caller must ensure that the address is properly aligned to the type `T`, and
/// that the address is not mutably aliased in the kernel. Failure to do so will
/// result in undefined behavior !
#[inline]
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn access<T, Object: FromBytes>(
    _phys: impl Into<Physical>,
    _f: impl FnOnce(&Object) -> T,
) -> T {
    unimplemented!()
}

/// Create a static mutable reference to the object located at the given physical
/// address.
///
/// This function can be extremely useful during the boot process, when the
/// kernel needs to allocate memory early on that will remain used for the
/// entire lifetime of the kernel.
///
/// # Safety
/// - The physical memory range should be valid and free to use.
/// - The physical memory start address should be properly aligned to the type `T`.
/// - After this function call, the physical memory range can only be referenced
/// through the returned reference. If the reference is lost, the physical
/// range will be leaked.
#[inline]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn leak<T: FromBytes>(_start: impl Into<Physical>) -> &'static mut T {
    unimplemented!()
}

/// Create a static mutable reference to the slice located at the given physical
/// address with `count` elements.
///
/// # Safety
/// - The physical memory range should be valid and free to use.
/// - The physical memory start address should be properly aligned to the type `T`.
/// - After this function call, the physical memory range can only be referenced
/// through the returned reference. If the reference is lost, the physical
/// range will be leaked.
#[inline]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn leak_slice<T: FromBytes>(
    _start: impl Into<Physical>,
    _count: usize,
) -> &'static mut [T] {
    unimplemented!()
}

/// Initialize an object at the given physical address with `obj` and create a
/// static mutable reference to it.
///
/// # Safety
/// - The physical memory range should be valid and free to use.
/// - The physical memory start address should be properly aligned to the type `T`.
/// - After this function call, the physical memory range can only be referenced
/// through the returned reference. If the reference is lost, the physical
/// range will be leaked and should never be used again. Failure to do so will
/// result in undefined behavior !
#[inline]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn init_and_leak<T>(_start: impl Into<Physical>, _obj: T) -> &'static mut T {
    unimplemented!()
}

/// Initialize a slice at the given physical address with `obj` for each `count`
/// elements, and create a static mutable reference to it.
///
/// # Safety
/// - The physical memory range should be valid and free to use.
/// - The physical memory start address should be properly aligned to the type `T`.
/// - After this function call, the physical memory range can only be referenced
/// through the returned reference. If the reference is lost, the physical memory
/// range will be leaked and should never be used again. Failure to do so will
/// result in undefined behavior !
#[inline]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn init_and_leak_slice<T: Copy>(
    _start: impl Into<Physical>,
    _count: usize,
    _obj: T,
) -> &'static mut [T] {
    unimplemented!()
}

/// Map a physical frame to a virtual address during the execution of the given
/// closure. The caller is responsible for ensuring that the virtual address will
/// be correctly used and that it will not break Rust's aliasing and mutability
/// rules.
#[inline]
pub fn map<T>(_frame: Frame, _f: impl FnOnce(Virtual) -> T) -> T {
    unimplemented!()
}
