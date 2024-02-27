#![cfg_attr(not(test), no_std)]
#![feature(negative_impls)]

pub mod boot;
pub mod cpu;
pub mod gdt;
pub mod idt;
pub mod io;
pub mod opcode;
pub mod percpu;
pub mod serial;
pub mod tss;

/// Initializes the `x86_64` architecture.
#[inline]
pub fn setup() {
    idt::setup();
    gdt::setup();
    tss::setup();
}
