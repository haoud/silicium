#![cfg_attr(not(test), no_std)]

pub mod cpu;
pub mod gdt;
pub mod io;
pub mod opcode;
pub mod serial;

/// Initializes the `x86_64` architecture.
#[inline]
pub fn setup() {
    gdt::setup();
}
