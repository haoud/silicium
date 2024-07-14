use crate::future::sleep::yield_now;
use core::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

/// A simple mutex implementation that allows exclusive access to the inner
/// data. If the lock is already held, the current task will be yielded until
/// the lock is acquired. The implementation is very simple and does not
/// support any kind of priority inversion prevention or fairness, as it does
/// not have a queue of waiting tasks, but it is enough for the current needs.
pub struct Mutex<T: ?Sized> {
    /// A flag to indicate if the mutex is locked or not
    lock: AtomicBool,

    /// The data that the mutex is protecting
    data: UnsafeCell<T>,
}

/// SAFETY: The `Mutex` type is safe to be send between threads as long
/// as the inner type is `Send`.
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

/// SAFETY: The `Mutex` type is safe to be shared between threads as long
/// as the inner type is `Send` because the inner type is protected by
/// the `Mutex` type against concurrent access.
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Create a new `Mutex` instance with the given data
    #[must_use]
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Consumes the mutex and returns the inner data
    #[must_use]
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: Sized> Mutex<T> {
    /// Lock the mutex. If the lock is already held, this function will
    /// yield the current task and try again until the lock is acquired.
    pub async fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            if let Some(lock) = self.try_lock() {
                break lock;
            }
            yield_now().await;
        }
    }

    /// Try to lock the mutex. If the lock is already held, this function
    /// will return `None`, otherwise it will return a guard that releases
    /// the lock when dropped.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.lock
            .compare_exchange_weak(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
            .then(|| Some(MutexGuard { mutex: self }))
            .unwrap_or(None)
    }
}

impl<T: Debug> Debug for Mutex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Mutex")
            .field("lock", &self.lock)
            .field("data", unsafe { &*self.data.get() })
            .finish()
    }
}

/// A guard that allows access to the inner data of the `Mutex` type, ensuring
/// that the lock is released when the guard is dropped. Since this object
/// can only be created by locking the `Mutex` type, it is guaranteed that
/// the lock is held while the guard is alive.
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}
