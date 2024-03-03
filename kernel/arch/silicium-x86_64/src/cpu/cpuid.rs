use bitflags::bitflags;
use core::arch::x86_64::CpuidResult;

pub type Result = CpuidResult;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Features : u64 {
        /// Floating-point unit on-chip
        const FPU = 1 << 0;

        /// Virtual 8086 mode enhancements
        const VME = 1 << 1;

        /// Debugging extensions
        const DE = 1 << 2;

        /// Page size extension
        const PSE = 1 << 3;

        /// Time stamp counter
        const TSC = 1 << 4;

        /// Model-specific registers
        const MSR = 1 << 5;

        /// Physical address extension
        const PAE = 1 << 6;

        /// Machine check exception
        const MCE = 1 << 7;

        /// CMPXCHG8B instruction
        const CX8 = 1 << 8;

        /// APIC on-chip
        const APIC = 1 << 9;

        /// SYSENTER/SYSEXIT instructions
        const SEP = 1 << 11;

        /// Memory type range registers
        const MTRR = 1 << 12;

        /// Page global bit
        const PGE = 1 << 13;

        /// Machine check architecture
        const MCA = 1 << 14;

        /// Conditional move and FCMOV instructions
        const CMOV = 1 << 15;

        /// Page attribute table
        const PAT = 1 << 16;

        /// 36-bit page size extension
        const PSE36 = 1 << 17;

        /// Processor serial number
        const PSN = 1 << 18;

        /// CLFLUSH instruction
        const CLFSH = 1 << 19;

        /// Debug store
        const DS = 1 << 21;

        /// Thermal monitor and software-controlled clock facilities
        const ACPI = 1 << 22;

        /// MMX technology
        const MMX = 1 << 23;

        /// FXSAVE/FXRSTOR instructions
        const FXSR = 1 << 24;

        /// Streaming SIMD extensions (SSE)
        const SSE = 1 << 25;

        /// Streaming SIMD extensions 2 (SSE2)
        const SSE2 = 1 << 26;

        /// Self-snoop
        const SS = 1 << 27;

        /// Hyper-threading technology
        const HTT = 1 << 28;

        /// Thermal monitor
        const TM = 1 << 29;

        /// Pending break enable
        const PBE = 1 << 31;

        /// Streaming SIMD extensions 3 (SSE3)
        const SSE3 = 1 << 32;

        /// PCLMULQDQ instruction (carry-less multiplication)
        const PCLMULQDQ = 1 << 33;

        /// 64-bit debug store
        const DTES64 = 1 << 34;

        /// MONITOR/MWAIT instructions
        const MONITOR = 1 << 35;

        /// CPL-qualified debug store
        const DS_CPL = 1 << 36;

        /// Virtual machine extensions
        const VMX = 1 << 37;

        /// Safer mode extensions
        const SMX = 1 << 38;

        /// Enhanced Intel SpeedStep technology
        const EST = 1 << 39;

        /// Thermal monitor 2
        const TM2 = 1 << 40;

        /// Supplemental SSE3 instructions
        const SSSE3 = 1 << 41;

        /// L1 context ID
        const CNXT_ID = 1 << 42;

        /// Silicon debug interface
        const SDBG = 1 << 43;

        /// Fused multiply-add (FMA3)
        const FMA = 1 << 44;

        /// CMPXCHG16B instruction
        const CMPXCHG16B = 1 << 45;

        /// xTPR update control
        const XTPR = 1 << 46;

        /// Perfmon and debug capability
        const PDCM = 1 << 47;

        /// Process-context identifiers
        const PCID = 1 << 48;

        /// Direct cache access
        const DCA = 1 << 50;

        /// Streaming SIMD extensions 4.1 (SSE4.1)
        const SSE4_1 = 1 << 51;

        /// Streaming SIMD extensions 4.2 (SSE4.2)
        const SSE4_2 = 1 << 52;

        /// Extended xAPIC support
        const X2APIC = 1 << 53;

        /// MOVBE instruction
        const MOVBE = 1 << 54;

        /// POPCNT instruction
        const POPCNT = 1 << 55;

        /// Time stamp counter deadline
        const TSC_DEADLINE = 1 << 56;

        /// AES instruction set
        const AES = 1 << 57;

        /// XSAVE/XRSTOR instructions
        const XSAVE = 1 << 58;

        /// OS-enabled extended feature
        const OSXSAVE = 1 << 59;

        /// Advanced Vector Extensions (AVX)
        const AVX = 1 << 60;

        /// 16-bit floating-point conversion instructions
        const F16C = 1 << 61;

        /// RDRAND instruction
        const RDRAND = 1 << 62;

        /// Hypervisor present (always 0 on physical CPUs)
        const HYPERVISOR = 1 << 63;
    }
}

/// Initialize the CPUID subsystem.
///
/// # Safety
/// This function is unsafe because it must only be called once and only during the
/// initialization of the kernel.
#[rustfmt::skip]
pub unsafe fn setup() {
    let vendor_bytes = vendor();
    let vendor = core::str::from_utf8(&vendor_bytes).unwrap_or("Unknown");

    log::trace!("CPUID: highest supported leaf is 0x{:08X}", leaf_max());
    log::trace!("CPUID: highest supported extended leaf is 0x{:08X}",leaf_extended_max());
    log::trace!("CPUID: vendor string is `{}`", vendor);
}

/// Return the highest supported extended CPUID leaf.
#[must_use]
pub fn leaf_extended_max() -> u32 {
    cpuid(0x8000_0000).eax
}

/// Return the highest supported CPUID leaf.
#[must_use]
pub fn leaf_max() -> u32 {
    cpuid(0).eax
}

/// Return the CPU vendor string.
#[must_use]
pub fn vendor() -> [u8; 12] {
    let cpuid = cpuid(0);
    [
        cpuid.ebx.to_le_bytes()[0],
        cpuid.ebx.to_le_bytes()[1],
        cpuid.ebx.to_le_bytes()[2],
        cpuid.ebx.to_le_bytes()[3],
        cpuid.edx.to_le_bytes()[0],
        cpuid.edx.to_le_bytes()[1],
        cpuid.edx.to_le_bytes()[2],
        cpuid.edx.to_le_bytes()[3],
        cpuid.ecx.to_le_bytes()[0],
        cpuid.ecx.to_le_bytes()[1],
        cpuid.ecx.to_le_bytes()[2],
        cpuid.ecx.to_le_bytes()[3],
    ]
}

/// Verify if the CPU supports the given feature.
#[must_use]
pub fn has_feature(feature: Features) -> bool {
    supported_features().contains(feature)
}

/// Return the CPU supported features.
#[must_use]
pub fn supported_features() -> Features {
    let cpuid = cpuid(1);
    Features::from_bits_truncate(u64::from(cpuid.edx) | (u64::from(cpuid.ecx) << 32))
}

/// Execute the `cpuid` instruction with the given leaf and return the result.
/// If the leaf is not supported, the result will be the highest supported leaf
/// on the current CPU.
#[must_use]
pub fn cpuid(leaf: u32) -> Result {
    // SAFETY: This is safe because the `cpuid` instruction is safe to call at
    // any time and is always present on x86_64.
    unsafe { core::arch::x86_64::__cpuid(leaf) }
}
