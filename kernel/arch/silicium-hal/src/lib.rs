#![cfg_attr(not(test), no_std)]
#![feature(negative_impls)]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
