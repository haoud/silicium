extern "C" {
    static __percpu_start: [usize; 0];
    static __percpu_end: [usize; 0];
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
