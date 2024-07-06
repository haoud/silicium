use arrayvec::ArrayVec;

pub mod allocator;
pub mod mmap;

#[derive(Debug)]
pub struct Info {
    /// The memory map of the system.
    pub mmap: ArrayVec<mmap::Entry, 32>,

    /// The number of bytes allocated by the boot allocator to
    /// correctly track the memory used by the kernel.
    pub boot_allocated: usize,
}
