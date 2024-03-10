#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "panic_info", feature(panic_info_message))]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
