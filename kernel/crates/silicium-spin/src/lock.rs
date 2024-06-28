use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

#[cfg(not(target_arch = "x86_64"))]
compile_error!(
    "The `silicium-spin` crate only supports the x86_64 architecture."
);

/// A simple spinlock implementation that allow exclusive access to the inner
/// data. The lock is acquired by spinning until it is free, and it is
/// released when the guard is dropped.
#[derive(Debug)]
pub struct Spinlock<T> {
    inner: UnsafeCell<T>,
    lock: AtomicBool,
}

/// SAFETY: The `Spinlock` type is safe to be send between threads as long
/// as the inner type is `Send`.
unsafe impl<T: Sized + Send> Send for Spinlock<T> {}

/// SAFETY: The `Spinlock` type is safe to be shared between threads as long
/// as the inner type is `Send`.
unsafe impl<T: Sized + Send> Sync for Spinlock<T> {}

impl<T> Spinlock<T> {
    /// Create a new spinlock with the given inner data. By default, the
    /// lock is unlocked.
    #[must_use]
    pub const fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
            lock: AtomicBool::new(false),
        }
    }

    /// Consumes the spinlock and returns the inner data.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    /// Check if the lock is held when the function is called.
    #[must_use]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    /// Lock the spinlock. This function will spin until the lock is acquired
    /// and will return a guard that releases the lock when dropped.
    #[must_use]
    pub fn lock(&self) -> SpinlockGuard<T> {
        loop {
            if let Some(lock) = self.try_lock_weak() {
                break lock;
            }
            core::hint::spin_loop();
        }
    }

    /// Disable IRQs and lock the spinlock. This function will disable IRQs,
    /// spin until the lock is acquired, and return a guard that releases the
    /// lock when dropped and restores the IRQ state.
    ///
    /// This should be used instead of `lock` when the lock can be acquired in
    /// a IRQ context, to avoid deadlocks. However, deadlocks can still occur
    /// if an exception is raised while the guard is held.
    #[must_use]
    pub fn lock_irq_safe(&self) -> SpinlockIrqGuard<T> {
        let irq = irq_save_and_disable();
        loop {
            if let Some(lock) = self.try_lock_weak_irq(irq) {
                break lock;
            }
            core::hint::spin_loop();
        }
    }

    /// Acquire the spinlock and call the given closure with a mutable
    /// reference to the inner data.
    #[inline]
    pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut *self.lock())
    }

    /// Acquire the spinlock and call the given closure with a mutable
    /// reference to the inner data. This function will disable IRQs before
    /// acquiring the lock and restore the IRQ state when the closure is done.
    ///
    /// This should be used instead of `with` when the lock can be acquired in
    /// a IRQ context, to avoid deadlocks. However, deadlocks can still occur
    /// if an exception is raised while the closure is running.
    #[inline]
    pub fn with_irq_safe<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut *self.lock_irq_safe())
    }

    /// Try to lock the spinlock. Returns `Some` if the lock was acquired, or
    /// `None` if the lock was not acquired. This version of the function may
    /// be a bit more efficient than `try_lock` because it uses a weaker memory
    /// ordering, but it may fail spuriously even if the lock is free.
    #[must_use]
    pub fn try_lock_weak(&self) -> Option<SpinlockGuard<T>> {
        self.lock
            .compare_exchange_weak(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
            .then(|| {
                // SAFETY: We have exclusive access to the lock.
                let inner = unsafe { &mut *self.inner.get() };
                Some(SpinlockGuard {
                    lock: &self.lock,
                    inner,
                })
            })
            .unwrap_or(None)
    }

    #[must_use]
    fn try_lock_weak_irq(&self, irq: bool) -> Option<SpinlockIrqGuard<T>> {
        self.lock
            .compare_exchange_weak(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
            .then(|| {
                // SAFETY: We have exclusive access to the lock.
                let inner = unsafe { &mut *self.inner.get() };
                Some(SpinlockIrqGuard {
                    lock: &self.lock,
                    inner,
                    irq,
                })
            })
            .unwrap_or(None)
    }

    /// Force unlocking the spinlock.
    ///
    /// # Safety
    /// This is *extremely* unsafe if the lock is not held by the current
    /// thread, but this can be useful in some cases, for example when a
    /// lock cannot be unlocked normally (FFI, panic...)
    #[inline]
    pub unsafe fn force_unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

/// A guard that releases the lock when dropped.
pub struct SpinlockGuard<'a, T> {
    lock: &'a AtomicBool,
    inner: &'a mut T,
}

impl<'a, T> Deref for SpinlockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}

/// A guard that releases the lock when dropped and restores the IRQ state
/// to the state before acquiring the lock.
pub struct SpinlockIrqGuard<'a, T> {
    lock: &'a AtomicBool,
    inner: &'a mut T,
    irq: bool,
}

impl<'a, T> Deref for SpinlockIrqGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> DerefMut for SpinlockIrqGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T> Drop for SpinlockIrqGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
        // SAFETY: Restoring the IRQ state here should be safe, we assume that
        // if the IRQ was enabled before acquiring the lock, it should be safe
        // to restore it here.
        unsafe {
            irq_restore(self.irq);
        }
    }
}

/// SAFETY: We implement the `RawMutex` trait from the `lock_api` crate, and
/// we must guarantee that the `Spinlock` type will provide the necessary
/// guarantees to have an exclusive access to the inner data.
unsafe impl lock_api::RawMutex for Spinlock<()> {
    type GuardMarker = lock_api::GuardSend;

    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self = Self::new(());

    /// Lock the spinlock. This function will spin until the lock is acquired.
    fn lock(&self) {
        // Prevent guard destructor running
        core::mem::forget(Self::lock(self));
    }

    /// Try to lock the spinlock. Returns `true` if the lock was acquired.
    fn try_lock(&self) -> bool {
        // Prevent guard destructor running
        Self::try_lock_weak(self).map(core::mem::forget).is_some()
    }

    /// Unlock the spinlock.
    ///
    /// # Safety
    /// The caller must ensure that the lock is held by the current thread
    /// when calling this function.
    unsafe fn unlock(&self) {
        self.force_unlock();
    }

    /// Check if the lock is held.
    fn is_locked(&self) -> bool {
        Self::is_locked(self)
    }
}

/// Save the current IRQ state and disable interrupts, returning the
/// previous state that can be used to restore the IRQ state.
#[inline]
#[must_use]
fn irq_save_and_disable() -> bool {
    // SAFETY: Reading the RFLAGS register is safe.
    let irq = unsafe {
        let mut flags: u32;
        core::arch::asm!("
            pushfq
            pop {0:r}",
            out(reg) flags
        );
        flags & (1 << 9) != 0
    };

    // SAFETY: Disabling interrupts is 100% safe.
    unsafe {
        core::arch::asm!("cli");
    }
    irq
}

/// Restore the IRQ state to the previous value.
///
/// # Safety
/// The caller must ensure that restoring the IRQ state is safe and will not
/// cause any undefined behavior or memory unsafety, especially when the IRQ
/// was enabled before calling `irq_save_and_disable`.
#[inline]
#[allow(clippy::undocumented_unsafe_blocks)]
unsafe fn irq_restore(state: bool) {
    match state {
        true => unsafe {
            core::arch::asm!("sti");
        },
        false => unsafe {
            core::arch::asm!("cli");
        },
    }
}
