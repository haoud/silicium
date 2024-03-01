use crate::opcode;
use bitflags::bitflags;

bitflags! {
    /// The features that can be enabled in the `xCR0` register.
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct Features: u64 {
        /// Enable using the x87 FPU state with the `xsave` and `xrstor`
        /// instructions. This flag is always set on modern processors.
        const X87 = 1 << 0;

        /// Enable using MXCSR and the XMM register with the `xsave` and
        /// `xrstor`instructions. This flag must be set if the `AVX` flag
        /// is set.
        const SSE = 1 << 1;

        /// Enable the AVX instruction set and using the upper 128 bits of
        /// AVX registers.
        const AVX = 1 << 2;
    }
}

/// Enable Xcr0 features.
///
/// # Safety
/// The caller must ensure that the xCR0 register exists and that the `xsetbv`
/// and `xgetbv` instructions are supported. The caller must also ensure that
/// enabling those features will not cause UB or memory unsafety.
pub unsafe fn enable(features: Features) {
    opcode::xsetbv(0, opcode::xgetbv(0) | features.bits());
}

/// Disable Xcr0 features.
///
/// # Safety
/// The caller must ensure that the xCR0 register exists and that the `xsetbv`
/// and `xgetbv` instructions are supported. The caller must also ensure that
/// Disabling those features will not cause UB or memory unsafety.
pub unsafe fn disable(features: Features) {
    opcode::xsetbv(0, opcode::xgetbv(0) & !features.bits());
}

/// Get the current Xcr0 features enabled.
///
/// # Safety
/// The caller must ensure that the xCR0 register exists and that the `xgetbv`
/// instruction is supported.
#[must_use]
pub unsafe fn current() -> Features {
    Features::from_bits_truncate(opcode::xgetbv(0))
}
