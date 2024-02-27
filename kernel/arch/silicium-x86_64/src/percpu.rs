use crate::{boot, msr};
use macros::init;

extern "C" {
    static __percpu_start: [usize; 0];
    static __percpu_end: [usize; 0];
}

/// Setup the per-CPU section for the current CPU.
///
/// # Safety
/// This function is unsafe because it must only be called once per core and only
/// during the initialization of the kernel. Failing to do so will result in
/// undefined behavior.
#[init]
pub unsafe fn setup() {
    // Compute some information about the per-CPU section
    let percpu_start = core::ptr::addr_of!(__percpu_start) as usize;
    let percpu_end = core::ptr::addr_of!(__percpu_end) as usize;
    let percpu_length = percpu_end - percpu_start;

    // Allocate a per-CPU section for the current code and copy original per-cpu
    // section to the allocated one
    let percpu = boot::allocate(percpu_length);
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
