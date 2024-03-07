use arch::{apic, cpu, irq, smp};

#[cfg(feature = "panic_info")]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    use arch::apic::local::{IpiDestination, IpiPriority};

    irq::disable();
    if smp::ap_booted() {
        // SAFETY: This is safe because we have ensured that the APs are booted, meaning
        // that they can safely receive IPIs without triple faulting.
        unsafe {
            apic::local::send_ipi(IpiDestination::AllExcludingSelf, IpiPriority::Nmi, 0x00);
        }
    }

    log::error!("The kernel has encountered a fatal error that it cannot recover from");
    log::error!("The kernel must stop to prevent further damage");

    if let Some(message) = info.message() {
        if let Some(location) = info.location() {
            log::error!("CPU {}: {} at {}", cpu::id(), message, location);
        } else {
            log::error!("{}", message);
        }
    }

    // Halt the CPU
    cpu::halt();
}

#[cfg(not(feature = "panic_info"))]
#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    use arch::apic::local::{IpiDestination, IpiPriority};

    irq::disable();
    if smp::ap_booted() {
        // SAFETY: This is safe because we have ensured that the APs are booted, meaning
        // that they can safely receive IPIs without triple faulting.
        unsafe {
            apic::local::send_ipi(IpiDestination::AllExcludingSelf, IpiPriority::Nmi, 0x00);
        }
    }

    // Halt the CPU
    cpu::halt();
}
