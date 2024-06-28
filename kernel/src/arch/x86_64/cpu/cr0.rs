use bitflags::bitflags;

bitflags! {
      /// The features that can be enabled in the `CR0` register.
      #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
      pub struct Features : u64 {
          /// Enables protected mode.
          const PE = 1 << 0;

          /// Enable monitor of the coprocessor.
          const MP = 1 << 1;

          /// Force all x87 FPU and MMX instructions to cause an #NE exception,
          /// allowing the software to emulate FPU/MMX/SSE/SSE2/SSE3
          /// instructions
          const EM = 1 << 2;

          /// When set, using x87 FPU or MMX instructions will cause an # NM
          /// exception. This is used to implement lazy FPU saving/restoring.
          const TS = 1 << 3;

          /// Indicates that the processor supports the 387DX math coprocessor
          /// instructions. On modern processors, this is always set and cannot
          /// be cleared.
          const ET = 1 << 4;

          /// Enable the native error reporting mechanism for x87 FPU errors.
          const NE = 1 << 5;

          /// When set, disables the rights of supervisor code to write into
          /// read-only pages.
          const WP = 1 << 16;

          /// Enables automatic usermode alignment checking if the RFLAGS.AC
          /// flag is also set.
          const AM = 1 << 18;

          /// Ignored on modern processors, used to control the write-back or
          /// write-through cache strategy.
          const NW = 1 << 29;

          /// Disable some processor cache (model-dependent).
          const CD = 1 << 30;

          /// Enable paging. This bit required the `Self::PG` bit to be set.
          /// This bit is also required to enable long mode.
          const PG = 1 << 31;
      }
}

/// Enable CR0 features.
///
/// # Safety
/// The caller must ensure that enabling those features will not cause UB or
/// memory unsafety.
pub unsafe fn enable(features: Features) {
    let mut cr0: u64;
    core::arch::asm!("mov {}, cr0", out(reg) cr0);
    cr0 |= features.bits();
    core::arch::asm!("mov cr0, {}", in(reg) cr0);
}

/// Disable CR0 features.
///
/// # Safety
/// The caller must ensure that disabling those features will not cause UB or
/// memory unsafety.
pub unsafe fn disable(features: Features) {
    let mut cr0: u64;
    core::arch::asm!("mov {}, cr0", out(reg) cr0);
    cr0 &= !features.bits();
    core::arch::asm!("mov cr0, {}", in(reg) cr0);
}

/// Read the CR0 register and return the enabled features.
#[must_use]
pub fn read() -> Features {
    let cr0: u64;
    // SAFETY: Reading the CR0 register is safe and should not cause any side
    // effects that could lead to undefined behavior.
    unsafe {
        core::arch::asm!("mov {}, cr0", out(reg) cr0);
    }
    Features::from_bits_truncate(cr0)
}
