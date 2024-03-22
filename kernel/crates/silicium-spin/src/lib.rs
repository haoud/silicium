//! A simple spinlock implementation for use in the kernel.
//! For simplicity, this implementation simply relies on the `spin` crate
//! and re-exports its types.
//! In the future, we may want to implement our own spinlock to reduce the
//! number of dependencies and to have more control over the implementation.
#![cfg_attr(not(test), no_std)]
#![allow(clippy::match_bool)]

pub mod lock;

pub use lock::{Spinlock, SpinlockGuard, SpinlockIrqGuard};
