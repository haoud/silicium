use bitflags::bitflags;

bitflags! {
    /// The features that can be enabled in the `CR4` register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Features: u64 {
        /// Enables virtual-8086 mode support with hardware-supported
        /// performance enhancements.
        const VME = 1 << 0;

        /// Enables protected-mode virtual interrupts.
        const PVI = 1 << 1;

        /// Restrict the use of RDTSC and RDTSCP instructions to privileged
        /// code.
        const TSD = 1 << 2;

        /// Enable debug extensions that enable I/O breakpoints capability and
        /// enforcement treatment of DR4 and DR5 as reserved.
        const DE = 1 << 3;

        /// Enable the use of 4 MB physical frames in protected mode. In long
        /// mode, this flags is simply ignored.
        const PSE = 1 << 4;

        /// Enable physical address extension and 2 Mb physical frames. This
        /// flag is required to be set in long mode.
        const PAE = 1 << 5;

        /// Enable machine check exception to occur.
        const MCE = 1 << 6;

        /// Enable the global pages feature, which allow to make the page
        /// translation inside the TLB global to all processes. Those pages
        /// translations are not flushed when changing the CR3 register.
        const PGE = 1 << 7;

        /// Enable the performance monitoring counter and the RDPMC
        /// instruction to be used at any privilege level.
        const PCE = 1 << 8;

        /// Enable the FXSAVE and FXRSTOR instructions to manage the FPU state.
        const OSFXSR = 1 << 9;

        /// Enable the SIMD floating point exception (#XF) for handling SIMD
        /// floating point error.
        const OSXMMEXCPT = 1 << 10;

        /// Prevent the execution of the SGDT, SIDT, SLDT, SMSW, and
        /// STR instructions in user mode software.
        const UMPI = 1 << 11;

        /// Enable level 5 paging.
        const LA57 = 1 << 12;

        /// Enable VMX instructions.
        const VMXE = 1 << 13;

        /// Enable SMX instructions.
        const SMXE = 1 << 14;

        /// Enable user software to read and write their own FS and GS
        /// segment base
        const FSGSBASE = 1 << 16;

        /// Enable process-context identifiers (PCIDs) to tag TLB entries.
        const PCIDE = 1 << 17;

        /// Enable extended processor state management instructions,
        /// including XSAVE, XRESTORE, and XSETBV/XGETBV.
        const OSXSAVE = 1 << 18;

        /// Prevent the execution of instructions that reside in user pages
        /// when the processor is in supervisor mode.
        const SMEP = 1 << 20;

        /// Enable restriction for supervisor-mode read and write access to
        /// user-mode pages: access to used-mode pages is denied when the AC
        /// flag in EFLAGS is clear.
        const SMAP = 1 << 21;

        /// Enable protection keys feature.
        const PKE = 1 << 22;

        /// Enable CET shadow stack.
        const CET = 1 << 23;

        /// Enables 4 level paging to associate each address with a
        /// protection key.
        const PKS = 1 << 24;
    }
}

/// Enable CR4 features.
///
/// # Safety
/// The caller must ensure that enabling those features will not cause UB or
/// memory unsafety.
pub unsafe fn enable(features: Features) {
    let mut cr4: u64;
    core::arch::asm!("mov {}, cr4", out(reg) cr4);
    cr4 |= features.bits();
    core::arch::asm!("mov cr4, {}", in(reg) cr4);
}

/// Disable CR4 features.
///
/// # Safety
/// The caller must ensure that disabling those features will not cause UB or
/// memory unsafety.
pub unsafe fn disable(features: Features) {
    let mut cr4: u64;
    core::arch::asm!("mov {}, cr4", out(reg) cr4);
    cr4 &= !features.bits();
    core::arch::asm!("mov cr4, {}", in(reg) cr4);
}

/// Read the CR4 register and return the current features enabled.
#[must_use]
pub fn read() -> Features {
    let cr4: u64;
    // SAFETY: Reading the CR4 register is safe and should not cause any side
    // effects that could lead to undefined behavior.
    unsafe {
        core::arch::asm!("mov {}, cr4", out(reg) cr4);
    }
    Features::from_bits_truncate(cr4)
}
