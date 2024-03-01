use crate::{
    cpu::{self, cpuid::Features, cr0, cr4, xcr0},
    opcode,
};
use macros::init;

/// The extended state of the CPU, which includes the x87 FPU, MMX, and SSE registers.
/// The size of the buffer is 4096 bytes, which should be the maximum size of the
/// extended state.
///
/// TODO: To reduce the memory footprint of the kernel, the size of the buffer should
/// be computed at runtime.
#[derive(Debug, Clone)]
#[repr(C, align(64))]
pub struct ExtendedState {
    buffer: [u8; 4096],
}

impl ExtendedState {
    /// Restore the extended state of the CPU from the buffer.
    pub fn xrstor(&self) {
        // SAFETY: The buffer is valid and contains the extended state of the CPU.
        // The `xrstor` instruction support was checked during the setup of the CPU.
        unsafe {
            opcode::xrstor(self.as_ptr());
        }
    }

    /// Save the extended state of the CPU into the buffer.
    pub fn xsave(&mut self) {
        // SAFETY: The buffer is valid and has enough space to store the extended
        // state of the CPU. The `xsave` instruction support was checked during
        // the setup of the CPU.
        unsafe {
            opcode::xsave(self.as_mut_ptr());
        }
    }

    /// Get a mutable pointer to the extended state buffer.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr()
    }

    /// Get a pointer to the extended state buffer.
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }
}

impl Default for ExtendedState {
    fn default() -> Self {
        Self { buffer: [0; 4096] }
    }
}

/// Setup the SIMD support and enable the x87 FPU and SSE for user-space
/// applications.
///
/// # Panics
/// This function will panic if the `xsave` instruction is not supported by the CPU.
///
/// # Safety
/// The caller must ensure that this function is only called during the initialization
/// of the kernel and called once per CPU core
#[init]
pub unsafe fn setup() {
    assert!(
        cpu::cpuid::has_feature(Features::XSAVE),
        "xsave instruction is not supported by the CPU !"
    );

    // Disable FPU emulation and enable monitor of the coprocessor
    cpu::cr0::disable(cr0::Features::EM);
    cpu::cr0::enable(cr0::Features::MP);

    // Enable x87 FPU and SSE with exception support
    cpu::cr4::enable(cr4::Features::OSXMMEXCPT | cr4::Features::OSXSAVE | cr4::Features::OSFXSR);
    cpu::xcr0::enable(xcr0::Features::X87 | xcr0::Features::SSE);
}
