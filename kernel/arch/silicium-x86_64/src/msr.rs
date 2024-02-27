/// Represents an MSR register. MSR registers are used to control
/// various features of the CPU, such as enabling/disabling
/// features, setting up performance counters, and more.
///
/// For more information about MSR registers, see the Intel
/// manual, volume 4, chapter 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Register(u32);

impl Register {
    pub const EFER: Self = Self(0xC000_0080);
    pub const STAR: Self = Self(0xC000_0081);
    pub const LSTAR: Self = Self(0xC000_0082);
    pub const CSTAR: Self = Self(0xC000_0083);
    pub const FMASK: Self = Self(0xC000_0084);
    pub const FS_BASE: Self = Self(0xC000_0100);
    pub const GS_BASE: Self = Self(0xC000_0101);
    pub const KERNEL_GS_BASE: Self = Self(0xC000_0102);
}

/// Write the given value to the given MSR.
///
/// # Safety
/// This function is unsafe because writing to an MSR can cause unexpected side effects and
/// potentially violate memory safety. It can also cause undefined behavior or memory
/// unsafety if the MSR is not supported by the CPU.
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn write(msr: Register, value: u64) {
    core::arch::asm!(
        "wrmsr",
        in("ecx") msr.0,
        in("eax") (value as u32),
        in("edx") (value >> 32),
    );
}

/// Read the current value of the given MSR.
///
/// # Safety
/// This function is unsafe because reading from an MSR can cause unexpected side effects and
/// potentially violate memory safety. It can also cause undefined behavior or memory
/// unsafety if the MSR is not supported by the CPU
#[must_use]
pub unsafe fn read(msr: Register) -> u64 {
    let low: u32;
    let high: u32;

    core::arch::asm!(
        "rdmsr",
        in("ecx") msr.0,
        out("eax") low,
        out("edx") high,
    );

    u64::from(high) << 32 | u64::from(low)
}
