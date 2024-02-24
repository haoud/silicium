#![cfg_attr(not(test), no_std)]

pub mod cpu;
pub mod io;
pub mod opcode;

/// Initializes the x86_64 architecture.
pub fn setup() {}
