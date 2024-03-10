//! A simple spinlock implementation for use in the kernel.
//! For simplicity, this implementation simply relies on the `spin` crate
//! and re-exports its types.
//! In the future, we may want to implement our own spinlock to reduce the
//! number of dependencies and to have more control over the implementation.
#![cfg_attr(not(test), no_std)]

pub use spin::{Lazy, Mutex as Spinlock, Once, RwLock as RwSpinlock};
