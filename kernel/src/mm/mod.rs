use macros::init;

pub mod heap;
pub mod physical;

/// Setup the memory management system
///
/// # Safety
/// This function is unsafe because this function should only be called once,
/// and only during the initialization of the kernel. Failing to do so will result
/// in undefined behavior.
#[init]
pub unsafe fn setup(info: &boot::Info) {
    physical::setup(info);
}
