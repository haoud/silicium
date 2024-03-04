#![cfg_attr(not(test), no_std)]

#[cfg(not(target_arch = "x86_64"))]
compile_error!("silicium-addr only supports x86_64 architecture");

pub mod frame;
pub mod phys;
pub mod virt;

pub use frame::Frame;
pub use phys::Physical;
pub use virt::Virtual;
