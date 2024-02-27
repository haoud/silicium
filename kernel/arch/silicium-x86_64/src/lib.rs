#![cfg_attr(not(test), no_std)]
#![feature(negative_impls)]

use macros::init;

pub mod boot;
pub mod cpu;
pub mod gdt;
pub mod idt;
pub mod io;
pub mod irq;
pub mod msr;
pub mod opcode;
pub mod percpu;
pub mod serial;
pub mod smp;
pub mod tss;

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
    idt::setup();
    idt::load();
    smp::setup();
    gdt::setup();
    tss::setup();
}
