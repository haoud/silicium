use crate::arch::x86_64::{cpu, opcode};
use macros::init;

/// Setup the Time Stamp Counter (TSC).
///
/// # Safety
/// This function should only be called once during the initialization of the
/// kernel.
#[init]
pub unsafe fn setup() {
    // Check if the TSC is invariant. If the CPU does not support the necessary
    // CPUID leaf, we will not be able to determine if the TSC is invariant.
    if cpu::cpuid::leaf_extended_max() < 0x8000_0007 {
        log::warn!("TSC: Cannot determine TSC invariance");
    } else {
        let invariant_tsc = cpu::cpuid(0x8000_0007).edx & (1 << 8) != 0;
        if invariant_tsc {
            log::trace!("TSC: Invariant TSC is supported");
        } else {
            log::trace!("TSC: Invariant TSC not supported");
        }
    }

    // Try to determine the TSC frequency. If the CPU does not support the
    // necessary CPUID leaf, we will not be able to determine the frequency.
    if cpu::cpuid::leaf_max() < 0x15 {
        log::trace!("TSC: No information about TSC frequency available");
    } else {
        let cpu::cpuid::Result {
            eax: denominator,
            ebx: numerator,
            ecx: crystal_frequency,
            ..
        } = cpu::cpuid(0x15);

        let rate = (crystal_frequency * numerator) / denominator;
        log::trace!("TSC: Frequency is {} Hz", rate);
    }
}

/// Read the Time Stamp Counter (TSC).
///
/// # Note
/// This function is an abstraction over the `rdtsc` opcode. However, the
/// `rdtsc` opcode is not a serializing instruction, and it does not necessarily
/// wait for previous instructions to complete before reading the TSC. Therefore,
/// it is possible that the TSC will be read before previous instructions have
/// completed. To ensure that the TSC is read after previous instructions have
/// completed, use the [`rdtscp`] function instead.
#[inline]
#[must_use]
pub fn rdtsc() -> u64 {
    opcode::rdtsc()
}

/// Read the Time Stamp Counter (TSC) and the processor ID.
///
/// This function does all the same things as `rdtsc`, but execute an additional
/// `cpuid` instruction to serialize the instruction stream. This can be useful
/// to ensure that the TSC is read after previous instructions have completed,
/// which is not guaranteed by the `rdtsc` instruction because it is not a
/// serializing instruction.
#[inline]
#[must_use]
pub fn rdtscp() -> u64 {
    _ = cpu::cpuid(0);
    opcode::rdtsc()
}
