#![cfg_attr(not(test), no_std)]
#![feature(panic_info_message)]
#![feature(negative_impls)]
#![feature(const_pin)]

use macros::init;

pub mod apic;
pub mod boot;
pub mod cpu;
pub mod gdt;
pub mod idt;
pub mod io;
pub mod irq;
pub mod msr;
pub mod opcode;
pub mod paging;
pub mod percpu;
pub mod pic;
pub mod pit;
pub mod serial;
pub mod simd;
pub mod smp;
pub mod tss;

/// Request for the `HHDM` (High Half Direct Mapping) feature. This will order Limine
/// to map all physical memory to the high half of the virtual address space, at a fixed
/// offset of `0xFFFF_8000_0000_0000`. However, `Reserved` and `Bad Memory` regions will
/// not be mapped into the HHDM region.
#[used]
static HHDM_REQUEST: limine::request::HhdmRequest = limine::request::HhdmRequest::new();

/// Initializes the `x86_64` architecture.
///
/// # Safety
/// This function is unsafe because it must only be called once and only during the
/// initialization of the kernel. Before calling this function, the boot memory
/// allocator must be initialized to allow this function to dynamically allocate
/// memory.
#[init]
pub unsafe fn setup() {
    percpu::setup(0);
    paging::setup();
    gdt::setup();
    idt::setup();
    idt::load();
    tss::setup();
    simd::setup();
    cpu::cpuid::setup();

    // Remap the PIC and disable it
    pic::remap_and_disable();

    // Setup the APIC, LAPIC and IOAPIC
    apic::setup();
    apic::local::setup();
    apic::io::setup();

    // Setup the APIC timer only on the boot CPU
    apic::local::timer::setup();

    smp::setup();
    paging::load_kernel_pml4();
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    use crate::apic::local::{IpiDestination, IpiPriority};

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
