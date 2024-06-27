use crate::arch::x86_64::{
    apic,
    apic::local::{IpiDestination, IpiPriority},
    cpu, smp,
};
use core::sync::atomic::AtomicBool;

/// This flag is used to prevent the kernel from panicking multiple times
/// in a row. This often means that the kernel has panicked in the panic
/// handler itself, indicating a critical error during the panic handling
/// process.
static PANICKED: AtomicBool = AtomicBool::new(false);

#[atomic]
#[panic_handler]
#[cfg(feature = "panic_info")]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    // If we have already panicked, we should not panic again because it
    // means that we have panicked in the panic handler itself.
    if PANICKED.swap(true, core::sync::atomic::Ordering::SeqCst) {
        cpu::halt();
    }

    if smp::ap_booted() {
        // SAFETY: This is safe because we have ensured that the APs are
        // booted, meaning that they can safely receive IPIs without
        // triple faulting.
        unsafe {
            apic::local::send_ipi(
                IpiDestination::AllExcludingSelf,
                IpiPriority::Nmi,
                0x02,
            );
        }
    }

    log::error!(
        "The kernel has encountered a fatal error that it cannot recover from"
    );
    log::error!("The kernel must stop to prevent further damage");

    if let Some(location) = info.location() {
        log::error!("CPU {}: {} at {}", cpu::id(), info.message(), location);
    } else {
        log::error!("{}", info.message());
    }

    // Halt the CPU
    cpu::halt();
}

#[atomic]
#[panic_handler]
#[cfg(not(feature = "panic_info"))]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    if smp::ap_booted() {
        // SAFETY: This is safe because we have ensured that the APs are
        // booted, meaning that they can safely receive IPIs without triple
        // faulting.
        unsafe {
            apic::local::send_ipi(
                IpiDestination::AllExcludingSelf,
                IpiPriority::Nmi,
                0x00,
            );
        }
    }

    // Halt the CPU
    cpu::halt();
}
