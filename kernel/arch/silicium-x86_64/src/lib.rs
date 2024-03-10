#![cfg_attr(not(test), no_std)]
#![feature(negative_impls)]
#![feature(new_uninit)]
#![feature(never_type)]

extern crate alloc;

pub mod apic;
pub mod bump;
pub mod cpu;
pub mod exception;
pub mod gdt;
pub mod idt;
pub mod io;
pub mod irq;
pub mod msr;
pub mod opcode;
pub mod paging;
pub mod percpu;
pub mod physical;
pub mod pic;
pub mod pit;
pub mod serial;
pub mod simd;
pub mod smp;
pub mod thread;
pub mod tsc;
pub mod tss;

/// Initializes the `x86_64` architecture.
///
/// # Safety
/// This function is unsafe because it must only be called once and only during the
/// initialization of the kernel. Before calling this function, the boot memory
/// allocator must be initialized to allow this function to dynamically allocate
/// memory.
#[macros::init]
pub unsafe fn setup() {
    // Initialized the per-cpu variable for this core
    percpu::setup(0);

    // Setup the pagingation
    paging::setup();

    // Create and load the GDT
    gdt::setup();

    // Create and load the IDT
    idt::setup();
    idt::load();

    // Insert the TSS into the GDT and load it
    tss::setup();

    // Setup the SIMD support
    simd::setup();

    // Setup the CPU identification
    cpu::cpuid::setup();

    // Setup the TSC
    tsc::setup();

    // Remap the PIC and disable it
    pic::remap_and_disable();

    // Setup the APIC, LAPIC and IOAPIC
    apic::setup();
    apic::local::setup();
    apic::io::setup();

    // Setup the APIC timer only on the boot CPU
    apic::local::timer::setup();

    // Start the APs
    smp::setup();

    // Load the kernel PML4
    paging::load_kernel_pml4();
}
