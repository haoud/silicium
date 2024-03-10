//! A sequential lock, also known as a seqlock, is a synchronization primitive that
//! allows for fast reads and writes to a shared data structure. The seqlock is
//! similar to a spinlock, but it allows for concurrent reads and writes to the
//! data structure.
//!
//! The seqlock is implemented using a sequence number that is incremented before
//! and after writing to the data structure. When reading the data structure, the
//! sequence number is checked to ensure that the data is consistent and was not
//! modified during the read. If the data is being written to, the reader will spin
//! until the write is complete. If the read was interleaved with a write, the reader
//! will retry the read until it is consistent.
//!
//! # Unsoudness
//! Currently, the implementation of the `SeqLock` is unsound, because it uses an
//! implementation that is not allowed in Rust. The implementation uses volatile
//! volatile reads and writes to the data structure. This is undefined behavior
//! in Rust, because it can lead to data races. However, the implementation
//! ensures that the data race will not be reflected to the user, because the
//! sequence number is used to ensure that the read is consistent and was not
//! modified during the read. The implementation is widely used in the Linux
//! kernel, and should be safe in practice. The unsoundness is a known issue, and
//! will be fixed in a future version of the crate, maybe when atomic memcpy will
//! be available in Rust.
#![cfg_attr(not(test), no_std)]

use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicUsize, Ordering},
};

/// A sequential lock
pub struct SeqLock<T> {
    data: UnsafeCell<T>,
    seq: AtomicUsize,
}

/// SAFETY: `SeqLock` is safe to send between threads if the data is `Send`.
unsafe impl<T: Send> Send for SeqLock<T> {}

/// SAFETY: `SeqLock` is safe to share between threads if the data is `Send`,
/// because the data is protected by a sequential lock that guarantees consistent
/// reads and writes.
unsafe impl<T: Send> Sync for SeqLock<T> {}

impl<T: Copy> SeqLock<T> {
    /// Create a new `SeqLock` with the given data
    #[must_use]
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            seq: AtomicUsize::new(0),
        }
    }

    /// Write data to the `SeqLock`. This function is extremely fast, and will
    /// complete in a few instructions without spinning.
    pub fn write(&self, data: T) {
        // Increment the sequence number to indicate that the data is being written. The
        // sequence number will become odd, and the reader will wait for it to become even
        // before reading the data.
        self.seq.fetch_add(1, Ordering::Relaxed);

        // Make sure that the write of the data happens after incrementing the sequence
        // number. Ideally, we would use `Acquire` ordering, but unfortunately the `Acquire`
        // ordering is not available for `store` operations.
        core::sync::atomic::fence(Ordering::Release);

        // Write the data using a volatile write, because the data may be concurrently
        // read by the reader.
        // SAFETY: Actually, the line below is UB ^^'
        unsafe {
            core::ptr::write_volatile(self.data.get(), data);
        }

        // Increment the sequence number to indicate that the data is written. The sequence
        // number will become even, and the reader will be able to read the data.
        self.seq.fetch_add(1, Ordering::Relaxed);
    }

    /// Read data from the `SeqLock`. This function reads the data volatilely, and will
    /// use the sequence number to ensure that the read is consistent and was not
    /// modified during the read. If the data is being written to, this function will
    /// spin until the write is complete.
    pub fn read(&self) -> T {
        loop {
            // Load the sequence number. We use the `Acquire` ordering to ensure
            // that the read of the data happens after the read of the sequence
            let seq = self.seq.load(Ordering::Acquire);

            // If the sequence number is odd, the data is being written to. We
            // need to wait for the writer to finish.
            if (seq & 1) == 1 {
                core::hint::spin_loop();
                continue;
            }

            // Read the data using a volatile read, because the data may be concurrently
            // modified by the writer.
            // SAFETY: Actually, the line below is UB ^^'...But works in practice,
            // and is widely used in the linux kernel.
            let data =
                unsafe { core::ptr::read_volatile::<MaybeUninit<T>>(self.data.get().cast()) };

            // Make sure that the second read of the sequence number happens after the
            // read of the data. Ideally, we would use `Release` ordering, but unfortunately
            // the `Release` ordering is not available for `load` operations.
            core::sync::atomic::fence(Ordering::Acquire);

            // SAFETY: If the sequence number is still the same, the read was consistent
            // so the data was not modified during the read, and we can safely assume it
            // is initialized.
            unsafe {
                if seq == self.seq.load(Ordering::Relaxed) {
                    return data.assume_init();
                }
            }
        }
    }

    /// Consumes the `SeqLock`, returning the inner data
    #[must_use]
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: Copy + Default> Default for SeqLock<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Copy + core::fmt::Debug> core::fmt::Debug for SeqLock<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SeqLock")
            .field("data", &self.read())
            .field("seq", &self.seq)
            .finish()
    }
}
