use super::irq;
use crate::arch::x86_64::{bump, msr};
use config::KSTACK_SIZE;
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};
use macros::init;

extern "C" {
    /// The start of the per-CPU section. This is a linker symbol that is used to
    /// get the start of the per-CPU section.
    static __percpu_start: [u64; 0];

    /// The end of the per-CPU section. This is a linker symbol that is used to
    /// get the end of the per-CPU section.
    static __percpu_end: [u64; 0];
}

/// A per-CPU variable. This is a wrapper around an `UnsafeCell` that allows
/// safe access to a variable that is unique to each CPU. This is done by
/// using the GS segment register to access the per-CPU section of memory and
/// by storing the per-CPU variable at a fixed offset from the GS segment using
/// a special linker section.
///
/// # Thread safety
/// Since each CPU has its own copy of the variable, it is safe to access it
/// from multiple threads running on the same CPU, because each thread will
/// access a different copy of the variable. However, it is not safe to access
/// directly the variable because an interrupt could occur and the interrupt
/// handler could try to access the same variable. To avoid this, this struct
/// will disable interrupts when accessing the variable and will restore the
/// previous interrupt state when the guard goes out of scope.
///
/// # Warning
/// This struct is not meant to be used directly. Instead, use the `#[percpu]`
/// attribute to create a per-CPU variable. This macro will wrap your variable
/// into a `PerCpu` struct and will put it in the correct linker section.
///
/// # Unsoudness
/// However, some unsoundness is possible if an exception occurs while accessing
/// the per-CPU variable. We can't prevent this from happening, and I'm still
/// trying to figure out how to handle this case. For now, the best thing to do
/// is to avoid using per-CPU variables in exception handlers.
/// Futhermore, it is unsound to use per-CPU variables before their initialization,
/// but this happens very early in the kernel initialization and should not be a
/// real problem. Since a kernel is not an ordinary program, I think it is reasonable
/// the tolerate this kind of unsoundness.
#[derive(Debug)]
pub struct PerCpu<T> {
    inner: UnsafeCell<T>,
}

/// SAFETY: This does not implement `Send` because it is not safe to send a per-CPU
/// variable to another CPU. This is because the variable is unique to each CPU and
/// sending it to another CPU will defeat the purpose of having a per-CPU variable
/// and will lead to undefined behavior.
impl<T> !Send for PerCpu<T> {}

/// SAFETY: Since each CPU has its own copy of the variable and we disable interrupts
/// while accessing the variable, it is safe to implement `Sync` for `PerCpu` since
/// data races are not possible.
unsafe impl<T> Sync for PerCpu<T> {}

impl<T> PerCpu<T> {
    /// Create a new per-CPU variable.
    ///
    /// # Safety
    /// This function is unsafe because it should not be called directly. Instead, use
    /// the `#[percpu]` attribute to create a per-CPU variable. This macro will wrap
    /// your variable into a `PerCpu` struct and will put it in the correct linker section.
    /// Failure to do so will result in undefined behavior when using the per-CPU variable.
    pub const unsafe fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    /// Get a reference to the per-CPU variable for the current CPU wrapped in a guard. The guard
    /// disable interrupts during its creation and will restore the previous interrupt state when
    /// it will go out of scope.
    ///
    /// # Safety
    /// This function is safe but can be unsound if the per-CPU area is not initialized before
    /// calling this function ! For simplicity, this function is marked as safe but this may be
    /// rewritten in the future to be unsafe.
    pub fn local(&self) -> PerCpuGuard<T> {
        // SAFETY: This is safe because we are sure that the GS segment is initialized and valid
        // and that no interrupt will occur while accessing the variable.
        unsafe { PerCpuGuard::new(self.get_unckecked()) }
    }

    /// Get a mutable reference to the per-CPU variable for the current CPU wrapped in a guard. The
    /// guard disable interrupts during its creation and will restore the previous interrupt state
    /// when it will go out of scope.
    ///
    /// # Safety
    /// This function is safe but can be unsound if the per-CPU area is not initialized before
    /// calling this function ! For simplicity, this function is marked as safe but this may be
    /// rewritten in the future to be unsafe.
    pub fn local_mut(&mut self) -> PerCpuGuardMut<T> {
        // SAFETY: This is safe because we are sure that the GS segment is initialized and valid
        // and that no interrupt will occur while accessing the variable.
        unsafe { PerCpuGuardMut::new(self.get_unckecked_mut()) }
    }

    /// Safely get a reference to the per-CPU variable during the execution of the given closure.
    /// During the execution of the closure, interrupts will be disabled to avoid data races and
    /// undefined behavior.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let guard = self.local();
        f(&*guard)
    }

    /// Safely get a mutable reference to the per-CPU variable during the execution of the given
    /// closure. During the execution of the closure, interrupts will be disabled to avoid data
    /// races and undefined behavior.
    pub fn with_mut<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.local_mut();
        f(&mut *guard)
    }

    /// Get a reference to the per-CPU variable for the current CPU without any guard.
    ///
    /// # Safety
    /// This function is unsafe for two reasons:
    /// - It doesn't check if the GS segment is initialized and valid
    /// - It doesn't disable interrupts while accessing the variable
    /// The caller must ensure that the GS segment is initialized and valid and that no
    /// interrupt will occur while have a reference to the per-CPU variable. Failing to do
    /// so will result in undefined behavior.
    pub unsafe fn get_unckecked(&self) -> &T {
        &*fetch_percpu_object(self.inner.get())
    }

    /// Get a mutable reference to the per-CPU variable for the current CPU without any guard.
    ///
    /// # Safety
    /// This function is unsafe for two reasons:
    /// - It doesn't check if the GS segment is initialized and valid
    /// - It doesn't disable interrupts while accessing the variable
    /// The caller must ensure that the GS segment is initialized and valid and that no
    /// interrupt will occur while have a reference to the per-CPU variable. Failing to do
    /// so will result in undefined behavior.
    pub unsafe fn get_unckecked_mut(&mut self) -> &mut T {
        &mut *fetch_percpu_object(self.inner.get())
    }
}

impl<T: Copy> PerCpu<T> {
    /// Replace the value of the per-CPU variable for the current CPU with the given value and
    /// return the old value.
    pub fn replace(&mut self, value: T) -> T {
        core::mem::replace(&mut self.local_mut(), value)
    }

    /// Set the value of the per-CPU variable for the current CPU to the given value.
    pub fn set(&mut self, value: T) {
        *self.local_mut() = value;
    }

    /// Get the value of the per-CPU variable for the current CPU.
    pub fn get(&self) -> T {
        *self.local()
    }
}

impl<T: Default> PerCpu<T> {
    /// Take the value of the per-CPU variable for the current CPU and replace it with
    /// the default value
    pub fn take(&mut self) -> T {
        core::mem::take(&mut self.local_mut())
    }
}

/// A guard that is used to access a per-CPU variable. This guard will disable interrupts
/// during its creation and will restore the previous interrupt state when it goes out of
/// scope to avoid being interrupted while accessing the per-CPU variable, which could lead
/// to undefined behavior.
#[derive(Debug)]
pub struct PerCpuGuard<'a, T> {
    irq_state: irq::State,
    inner: &'a T,
}

impl<'a, T> PerCpuGuard<'a, T> {
    /// Create a new per-CPU guard.
    unsafe fn new(inner: &'a T) -> Self {
        let irq_state = irq::save_and_disable();
        Self { irq_state, inner }
    }

    /// Get a raw pointer to the per-CPU variable for the current CPU.
    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        core::ptr::addr_of!(*self.inner)
    }
}

impl<'a, T> Deref for PerCpuGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> Drop for PerCpuGuard<'a, T> {
    fn drop(&mut self) {
        irq::restore(core::mem::take(&mut self.irq_state));
    }
}

/// A mutable guard that is used to access a per-CPU variable. This guard will disable
/// interrupts during its creation and will restore the previous interrupt state when it
/// goes out of scope to avoid being interrupted and creating multiples mutables references
/// while accessing the per-CPU variable, which could lead to undefined behavior.
#[derive(Debug)]
pub struct PerCpuGuardMut<'a, T> {
    irq_state: irq::State,
    inner: &'a mut T,
}

impl<'a, T> PerCpuGuardMut<'a, T> {
    /// Create a new per-CPU mutable guard.
    unsafe fn new(inner: &'a mut T) -> Self {
        let irq_state = irq::save_and_disable();
        Self { inner, irq_state }
    }

    /// Get a raw mutable pointer to the per-CPU variable for the current CPU.
    #[must_use]
    pub fn as_ptr_mut(&mut self) -> *mut T {
        core::ptr::addr_of_mut!(*self.inner)
    }

    /// Get a raw pointer to the per-CPU variable for the current CPU.
    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        core::ptr::addr_of!(*self.inner)
    }
}

impl<'a, T> Deref for PerCpuGuardMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> DerefMut for PerCpuGuardMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T> Drop for PerCpuGuardMut<'a, T> {
    fn drop(&mut self) {
        irq::restore(core::mem::take(&mut self.irq_state));
    }
}

/// Setup the per-CPU section for the current CPU.
///
/// # Safety
/// This function is unsafe because it must only be called once per core and only
/// during the initialization of the kernel. Failing to do so will result in
/// undefined behavior.
#[init]
pub unsafe fn setup(lapic_id: u64) {
    // Compute some information about the per-CPU section
    let percpu_start = core::ptr::addr_of!(__percpu_start) as usize;
    let percpu_end = core::ptr::addr_of!(__percpu_end) as usize;
    let percpu_length = percpu_end - percpu_start;

    // Allocate a per-CPU section for the current code and copy original per-cpu
    // section to the allocated one
    let percpu = bump::boot_allocate(percpu_length);
    core::ptr::copy_nonoverlapping(percpu_start as *const u8, percpu, percpu_length);

    // Set the GS base to the allocated per-CPU section. We set the GS_BASE MSR and
    // not the KERNEL_GS_BASE MSR because the active GS base is always loaded from
    // the GS_BASE MSR. The KERNEL_GS_BASE MSR is only used to store the GS base
    // when the kernel is running in user mode.
    msr::write(msr::Register::KERNEL_GS_BASE, 0);
    msr::write(msr::Register::GS_BASE, percpu as u64);

    // Store the per-CPU section base in the GS:0 location to easily access it
    // when the code needs to access the per-CPU section
    core::arch::asm!("mov gs:0, {}", in(reg) percpu);

    // Store the LAPIC ID in the GS:8 location to easily access it when the code
    // needs to access the LAPIC ID
    core::arch::asm!("mov gs:8, {}", in(reg) lapic_id);
}

/// Allocate a kernel stack for the current CPU and initialize the GS:16 location
/// with the freshly allocated stack.
/// This kernel stack will be used when the user code enter into kernel mode, after
/// saving its state on the small stack located in the TSS.
/// This allow to have a small kernel stack for each thread and use a bigger kernel
/// stack per core, saving memory and avoiding stack overflow.
///
/// # Safety
/// This function should only be called once per core and only during the initialization
/// of the kernel. Failing to do so will result in undefined behavior.
#[init]
pub unsafe fn setup_kernel_stack() {
    let kstack = bump::boot_allocate(KSTACK_SIZE);
    let rsp = kstack.byte_add(KSTACK_SIZE).cast::<usize>();
    set_kernel_stack(rsp);
}

/// Fetch the per-CPU object for the current CPU for the static variable located at
/// the given address. This function will correctly return an unique object for each
/// CPU using the GS segment register.
///
/// # Safety
/// This function is unsafe because it needs multiples conditions to work properly:
/// - The pointer must be a valid pointer to an static variable located in
/// the per-CPU section
/// - The GS segment must be initialized and valid
/// - This function should not be interrupted during its execution
#[must_use]
pub unsafe fn fetch_percpu_object<T>(ptr: *mut T) -> *mut T {
    let percpu_start = core::ptr::addr_of!(__percpu_start) as usize;
    let offset = ptr as usize - percpu_start;
    let percpu = get_percpu_section();

    (percpu + offset) as *mut T
}

/// Get the percpu base for the current CPU. The percpu base is relatic to the GS segment
/// and is stored in the GS:0 location.
///
/// # Safety
/// This function is unsafe because it reads the percpu base relative to the GS segment
/// register. The GS segment must contain a valid address that this function can safely
/// read from. Failing to do so will result in undefined behavior.
#[must_use]
pub unsafe fn get_percpu_section() -> usize {
    let percpu: usize;
    core::arch::asm!("mov {}, gs:0", out(reg) percpu);
    percpu
}

/// Set the kernel stack pointer that will be used when the user code will trigger an
/// system call.
///
/// # Safety
/// The caller must ensure that the given stack pointer is a valid stack pointer and
/// points to a valid stack that is big enough to be used as a kernel stack. The
/// stack must remain valid while this stack pointer is used as the kernel stack.
/// ***Please remember when passing the rsp argument that the stack grows downwards !***
pub unsafe fn set_kernel_stack(rsp: *mut usize) {
    core::arch::asm!("mov gs:24, {}", in(reg) rsp as u64);
}
