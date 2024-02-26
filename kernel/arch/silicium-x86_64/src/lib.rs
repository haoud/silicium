#![cfg_attr(not(test), no_std)]
#![feature(asm_const)]
#![feature(naked_functions)]

pub mod cpu;
pub mod gdt;
pub mod idt;
pub mod io;
pub mod opcode;
pub mod serial;
pub mod tss;

/// Initializes the `x86_64` architecture.
#[inline]
pub fn setup() {
    idt::setup();
    gdt::setup();
    tss::setup();
}
