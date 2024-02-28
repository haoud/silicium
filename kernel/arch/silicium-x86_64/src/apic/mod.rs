use macros::init;

pub mod io;
pub mod local;

/// Setup the APIC
///
/// # Safety
/// This function is unsafe because it must only be called once, before
/// initialized other cores. It should also only be called during the kernel
/// initialization.
#[init]
pub unsafe fn setup() {
    // TODO: Remap the APIC MMIO
}
