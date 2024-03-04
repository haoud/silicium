#![cfg_attr(not(test), no_std)]
pub use arrayvec::ArrayVec;

pub mod mmap;

pub struct Info {
    pub mmap: ArrayVec<mmap::Entry, 32>,
}
