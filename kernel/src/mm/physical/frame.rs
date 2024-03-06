use bitflags::bitflags;

bitflags! {
    /// Flags that can be set on a frame.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags : u32 {
        /// If set, the frame is poisoned and should not be used for any purpose.
        const POISONED = 1 << 0;

        /// If set, the frame is reserved by the hardware and should not be used for
        /// any purpose, including allocation.
        const RESERVED = 1 << 1;

        /// If set, the frame is regular memory and can be used for allocation.
        const REGULAR = 1 << 2;

        /// If set, the frame is used by the kernel. This is used to track which frames
        /// are used by the kernel.
        const KERNEL = 1 << 3;

        /// If set, the frame is used by the bootloader. This is used to track which frames
        /// are used by the bootloader and can be reclaimed by the kernel when there are no
        /// longer needed.
        const BOOT = 1 << 4;

        /// If set, the frame is free and can be used for allocation.
        const FREE = 1 << 5;
    }
}

/// Information about a frame. It contains the flags set on the frame
/// and the reference count (only meaningful if the frame is regular
/// and not free).
#[derive(Debug)]
pub struct Info {
    pub(super) flags: Flags,
    pub(super) count: u32,
}

impl Info {
    /// Creates a new Poisoned frame info.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            flags: Flags::POISONED,
            count: 0,
        }
    }

    /// Retain the frame, incrementing the reference count.
    ///
    /// If the reference count reach the maximum value of a `u32`, this function will print a
    /// warning message and saturate the reference count at the maximum value of a `u32`. Therefore,
    /// the frame will be pinned in memory indefinitely.
    pub fn retain(&mut self) {
        if self.count == (u32::MAX - 1) {
            log::warn!("Frame reference count overflow: frame pinned in memory indefinitely");
        }
        self.count = self.count.saturating_add(1);
    }

    /// Decrements the reference count of the frame. If the reference count reaches zero, the function
    /// will return `true`, indicating that the frame can be freed.
    ///
    /// If the reference count is equal to the maximum value of a `u32`, this function will **not**
    /// decrement the reference count.
    ///
    /// This is because the `retain` function will saturate the reference count at the maximum
    /// value of a `u32`, but this lose the information of how many times the frame has been
    /// retained. By not decrementing the reference count, we assure that the frame will not
    /// be freed while still in use.
    ///
    /// # Panics
    /// Panics if the reference count is already zero, as this indicates a double free. This
    /// is a programming error and should be fixed as soon as possible.
    pub fn release(&mut self) -> bool {
        assert!(self.count > 0);
        if self.count != u32::MAX {
            self.count -= 1;
        }
        self.count == 0
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}
