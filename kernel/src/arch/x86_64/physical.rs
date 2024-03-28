use addr::{Frame, Physical, Virtual};
use zerocopy::FromBytes;

/// The start of the HHDM region. Since the kernel does not use the 5 level paging, the
/// HHDM region starts at `0xFFFF_8000_0000_0000`. In theory, we should use the value
/// given by Limine in the HHDM response but with the current implementation, the value
/// is always `0xFFFF_8000_0000_0000`.
const HHDM_START: Virtual = Virtual::new(0xFFFF_8000_0000_0000);

/// Obtain an mutable reference to the object located at the given physical address
/// during the execution of the given closure.
///
/// # Safety
/// The caller must ensure that the address is properly aligned to the type `T`, and
/// that the address is not aliased anywhere in the kernel. Failure to do so will
/// result in undefined behavior !
#[inline]
pub unsafe fn access_mut<T, Object: FromBytes>(
    phys: impl Into<Physical>,
    f: impl FnOnce(&mut Object) -> T,
) -> T {
    f(&mut *translate(phys).as_mut_ptr())
}

/// Obtain a reference to the object located at the given physical address during
/// the execution of the given closure.
///
/// # Safety
/// The caller must ensure that the address is properly aligned to the type `T`, and
/// that the address is not mutably aliased in the kernel. Failure to do so will
/// result in undefined behavior !
#[inline]
pub unsafe fn access<T, Object: FromBytes>(
    phys: impl Into<Physical>,
    f: impl FnOnce(&Object) -> T,
) -> T {
    f(&*translate(phys).as_ptr())
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
pub unsafe fn leak<T: FromBytes>(start: impl Into<Physical>) -> &'static mut T {
    &mut *translate(start).as_mut_ptr()
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
pub unsafe fn leak_slice<T: FromBytes>(
    start: impl Into<Physical>,
    count: usize,
) -> &'static mut [T] {
    core::slice::from_raw_parts_mut(translate(start).as_mut_ptr(), count)
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
pub unsafe fn init_and_leak<T>(start: impl Into<Physical>, obj: T) -> &'static mut T {
    let ptr = translate(start).as_mut_ptr::<T>();
    ptr.write(obj);
    &mut *ptr
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
pub unsafe fn init_and_leak_slice<T: Copy>(
    start: impl Into<Physical>,
    count: usize,
    obj: T,
) -> &'static mut [T] {
    let ptr = translate(start).as_mut_ptr::<T>();
    for i in 0..count {
        ptr.add(i).write(obj);
    }
    core::slice::from_raw_parts_mut(ptr, count)
}

/// Map a physical frame to a virtual address during the execution of the given
/// closure. The caller is responsible for ensuring that the virtual address will
/// be correctly used and that it will not break Rust's aliasing and mutability
/// rules.
#[inline]
pub fn map<T>(frame: Frame, f: impl FnOnce(Virtual) -> T) -> T {
    f(translate(frame))
}

/// Translate a physical address to a virtual address. On this architecture, the
/// translation is very simple since the kernel uses a direct mapping of all the
/// physical memory, mapped to an fixed virtual address.
///
/// However, the caller must ensure to correctly use the returned virtual address
/// and to no break Rust's aliasing and mutability rules.
#[inline]
#[must_use]
pub fn translate(phys: impl Into<Physical>) -> Virtual {
    // SAFETY: This is safe since the HHDM_START is a valid canonical address, and in
    // the `x86_64` architecture, the physical address is at most 52 bits. Therefore,
    // the addition of the physical address to the HHDM_START will always result in a
    // valid canonical address.
    unsafe { Virtual::new_unchecked(usize::from(HHDM_START) + usize::from(phys.into())) }
}
