#![cfg_attr(not(test), no_std)]
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
