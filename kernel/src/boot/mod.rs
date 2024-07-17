use arrayvec::ArrayVec;

pub mod allocator;
pub mod mmap;

#[derive(Debug)]
pub struct Info {
    /// The memory map of the system.
    pub mmap: ArrayVec<mmap::Entry, 32>,
}
