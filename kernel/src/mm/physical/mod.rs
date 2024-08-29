use crate::{
    arch::{
        self,
        x86_64::addr::{Frame, Physical},
    },
    boot,
};

use config::PAGE_SIZE;
use macros::init;

pub mod allocator;
pub mod frame;

/// The global allocator for the physical memory manager. This is used to
/// allocate and deallocate contiguous physical memory regions.
pub static ALLOCATOR: spin::Mutex<allocator::Allocator> =
    spin::Mutex::new(allocator::Allocator::new());

/// The global state of the physical memory manager. This is used to track
/// the state of each frame in the system.
pub static STATE: spin::Mutex<State> = spin::Mutex::new(State::uninitialized());

/// The state of the physical memory manager. This is used to track the state
/// of each frame in the system. It contains an array of `frame::Info` that is
/// used to track the state of each frame in the system, and also contains
/// some statistics about the memory usage of the system.
pub struct State {
    frames: &'static mut [frame::Info],
}

impl State {
    /// Creates a new uninitialized state.
    #[must_use]
    pub const fn uninitialized() -> Self {
        Self { frames: &mut [] }
    }

    /// Creates a new state from the given memory map. It will allocate an
    /// array of `frame::Info` and initialize it with the given memory map.
    #[must_use]
    pub fn new(info: &boot::Info) -> Self {
        let mmap = &info.mmap;
        let last_frame = mmap
            .iter()
            .filter(|entry| entry.kind.regular_memory())
            .map(|entry| entry.end())
            .max()
            .expect("No usable memory regions found");

        let array_size = (usize::from(last_frame) / usize::from(PAGE_SIZE))
            * core::mem::size_of::<frame::Info>();

        let array_location = mmap
            .iter()
            .filter(|entry| entry.kind == boot::mmap::Kind::Usable)
            .find(|entry| entry.length >= array_size + 4096)
            .map(|entry| Frame::new(usize::from(entry.start)))
            .expect("No suitable memory region found for frame infos");

        let array_count = array_size / core::mem::size_of::<frame::Info>();

        log::trace!(
            "Physical: Frame info array location: {:?}",
            array_location
        );
        log::trace!(
            "Physical: Frame info array size: {} KiB",
            array_size / 1024
        );

        // Initialize the frame info array with default values and create it
        // from the computed location and size
        // SAFETY: This is sae because the memory region is valid and free to
        // use, and is properly aligned to the type `frame::Info`.
        let array = unsafe {
            arch::physical::init_and_leak_slice(
                array_location,
                array_count,
                frame::Info::default(),
            )
        };

        let mut poisoned = array.len();
        let mut reserved = 0;
        let mut kernel = 0;
        let mut boot = 0;
        let mut free = 0;

        // Initialize the frame info array with the given memory map and
        // update the statistics about the memory usage of the system.
        for entry in mmap {
            for frame in array
                .iter_mut()
                .take(Self::frame_info_index(entry.end()))
                .skip(Self::frame_info_index(entry.start))
            {
                frame.flags &= !frame::Flags::POISONED;
                poisoned -= 1;

                match entry.kind {
                    boot::mmap::Kind::Usable => {
                        frame.flags |= frame::Flags::REGULAR;
                        frame.flags |= frame::Flags::FREE;
                        free += 1;
                    }
                    boot::mmap::Kind::AcpiReclaimable => {
                        frame.flags |= frame::Flags::REGULAR;
                        frame.flags |= frame::Flags::KERNEL;
                        kernel += 1;
                    }
                    boot::mmap::Kind::BootloaderReclaimable => {
                        frame.flags |= frame::Flags::BOOT;
                        frame.count = 1;
                        boot += 1;
                    }
                    boot::mmap::Kind::Kernel => {
                        frame.flags |= frame::Flags::KERNEL;
                        frame.count = 1;
                        kernel += 1;
                    }
                    boot::mmap::Kind::Reserved => {
                        frame.flags |= frame::Flags::RESERVED;
                        reserved += 1;
                    }
                    boot::mmap::Kind::BadMemory => {
                        frame.flags |= frame::Flags::POISONED;
                        poisoned += 1;
                    }
                }
            }
        }

        // Mark the frame used by the array as used by the kernel. This is
        // done to prevent the frame used by the info array from being used
        // for allocation
        let count = (array_size / usize::from(PAGE_SIZE)) + 1;
        let start = array_location.index();
        for frame in array.iter_mut().take(start + count).skip(start) {
            frame.flags &= !frame::Flags::FREE;
            frame.flags |= frame::Flags::KERNEL;
            frame.count = 1;
            kernel += 1;
            free -= 1;
        }

        log::info!(
            "Physical: {} KiB poisoned, {} KiB reserved, {} KiB kernel, {} KiB bootloader, {} KiB free",
            (poisoned * usize::from(PAGE_SIZE)) / 1024,
            (reserved * usize::from(PAGE_SIZE)) / 1024,
            (kernel * usize::from(PAGE_SIZE)) / 1024,
            (boot * usize::from(PAGE_SIZE)) / 1024,
            (free * usize::from(PAGE_SIZE)) / 1024
        );

        State { frames: array }
    }

    /// Return a mutable slice to the state of physical frames in the system.
    #[must_use]
    pub fn frames_info_mut(&mut self) -> &mut [frame::Info] {
        self.frames
    }

    /// Returns a slice to the state of physical frames in the system.
    #[must_use]
    pub fn frames_info(&self) -> &[frame::Info] {
        self.frames
    }

    /// Returns the index of the frame info that contains the given physical
    /// address. If the address does not exists in the system, the index is
    /// invalid and should not be used.
    pub fn frame_info_index(frame: Physical) -> usize {
        usize::from(frame) / usize::from(PAGE_SIZE)
    }
}

/// Initializes the physical memory manager with the given memory map.
///
/// # Safety
/// This function is unsafe because it can cause undefined behavior if the
/// memory map is invalid or does not include the memory used by the kernel
/// in a region that must not be used for allocation. The caller must also
/// ensure that this function is called only once and during the initialization
/// of the kernel.
#[init]
pub unsafe fn setup(info: &boot::Info) {
    *STATE.lock() = State::new(info);
}
