use crate::{
    arch,
    mm::physical::{self, allocator::Flags},
};
use config::PAGE_SHIFT;

/// The global heap allocator. This allocator is used to allocate memory on
/// the kernel heap. However, the kernel heap should only used to allocate
/// relatively small chunks of memory. Large allocations should be done using
/// the virtual memory allocator (not yet implemented).
#[global_allocator]
#[cfg(not(test))]
static ALLOCATOR: talc::Talck<spin::Mutex<()>, OomHandler> =
    talc::Talck::new(talc::Talc::new(OomHandler {}));

/// The global OOM handler when the kernel heap is exhausted. This handler will
/// allocate enough physical memory to satisfy the allocation request. If the
/// system is out of memory, the kernel will panic.
///
/// FIXME: The kernel should not panic when the system is out of memory.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct OomHandler {}

impl OomHandler {
    /// The size of the allocation that will be attempted when handling an OOM.
    const ALLOCATION_SIZE: usize = 128 * 1024;
}

impl talc::OomHandler for OomHandler {
    fn handle_oom(
        talc: &mut talc::Talc<Self>,
        layout: core::alloc::Layout,
    ) -> Result<(), ()> {
        // The heap should not be used to allocate large chunks of memory.
        // Large allocations should be done using the virtual memory allocator
        // (not yet implemented)
        if layout.size() > Self::ALLOCATION_SIZE {
            return Err(());
        }

        // Allocate 128KiB of contiguous physical memory
        let count = Self::ALLOCATION_SIZE >> PAGE_SHIFT;
        let frames = physical::ALLOCATOR
            .lock()
            .allocate_range(count, Flags::KERNEL)
            .ok_or(())?;

        // Convert the physical address to a virtual address
        let memory = unsafe {
            arch::physical::leak_slice::<u8>(frames, Self::ALLOCATION_SIZE)
        };
        let start = memory.as_mut_ptr();
        let end =
            unsafe { memory.as_mut_ptr().byte_add(Self::ALLOCATION_SIZE) };

        // SAFETY: The given span is valid, does not overlapp with any other
        // span, is not in use anywhere else in the system and is valid for
        // reads and writes.
        unsafe { talc.claim(talc::Span::new(start, end)).map(|_| ()) }
    }
}
