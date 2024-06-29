use crate::{
    arch::x86_64::addr::Frame,
    mm::physical::{frame, STATE},
};

pub struct Allocator;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags: u8 {
        /// If set, the frame is used by the kernel, otherwise it is used
        /// by userspace. This is used to track which frames are used by
        /// the kernel.
        const KERNEL = 1 << 0;
    }
}

impl Allocator {
    /// Create a new instance of the frame allocator
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Allocates a free frame from the frame state. Returns `None` if no
    /// frame is available, or a frame if a frame was successfully allocated.
    #[must_use]
    pub fn allocate(&mut self, flags: Flags) -> Option<Frame> {
        self.allocate_range(1, flags)
    }

    /// Allocates a range of free frames from the frame state. Returns `None`
    /// if no frame is available, or a range of owned frames if a range of
    /// frames was successfully allocated.
    #[must_use]
    pub fn allocate_range(
        &mut self,
        count: usize,
        flags: Flags,
    ) -> Option<Frame> {
        let mut state = STATE.lock();
        let frames = state.frames_info_mut();

        // Find the first range of contiguous free frames. If there is no such
        // range, return `None`
        let index = frames
            .windows(count)
            .enumerate()
            .find(|(_, frames)| {
                frames
                    .iter()
                    .all(|frame| frame.flags.contains(frame::Flags::FREE))
            })
            .map(|(index, _)| index)?;

        // Mark the frames as used and set the kernel flag if requested
        frames[index..(index + count)].iter_mut().for_each(|frame| {
            frame.flags.remove(frame::Flags::FREE);
            if flags.contains(Flags::KERNEL) {
                frame.flags.insert(frame::Flags::KERNEL);
            }
        });

        Some(Frame::from_index(index))
    }

    /// Deallocates a frame.
    ///
    /// # Safety
    /// The caller must ensure that the frame is not used by the kernel or any
    /// other part of the system, and that the frame was not already
    /// deallocated. Failure to do so will cause undefined behavior.
    pub unsafe fn deallocate(&mut self, frame: Frame) {
        self.deallocate_range(frame, 1);
    }

    /// Deallocates a range of frames.
    ///
    /// # Safety
    /// The caller must ensure that the range of frames is not used by the
    /// kernel or any other part of the system, and that the range of frames
    /// was not already deallocated. Failure to do so will cause undefined
    /// behavior.
    pub unsafe fn deallocate_range(&mut self, frame: Frame, count: usize) {
        let index = frame.index();

        if let Some(frame_range) = STATE
            .lock()
            .frames_info_mut()
            .get_mut(index..(index + count))
        {
            for frame in frame_range.iter_mut() {
                frame.flags.remove(frame::Flags::KERNEL);
                frame.flags.insert(frame::Flags::FREE);
            }
        } else {
            log::warn!(
                "Deallocating invalid frame range: {} frames starting at {}",
                count,
                frame
            );
        }
    }

    /// Retains a frame, incrementing the reference count of the frame.
    /// This is used to prevent the frame from being deallocated while it
    /// is in use: the frame will need one more call to [`deallocate`] or
    /// [`deallocate_range`] to be effectively deallocated.
    ///
    /// # Safety
    /// The frame must have been previously allocated by [`allocate`] or
    /// [`allocate_range`], and must not have been deallocated. Failure
    /// to do so will cause undefined behavior.
    pub unsafe fn reference(&mut self, frame: Frame) {
        self.reference_range(frame, 1);
    }

    /// Retains a range of frames, incrementing the reference count of each
    /// frame. This is used to prevent the frame from being deallocated while
    /// it is in use: the frames will need one more call to [`deallocate`] or
    /// [`deallocate_range`] to be effectively deallocated.
    ///
    /// # Safety
    /// The range of frame must have been previously allocated by
    /// [`allocate_range`] or [`allocate`], and must not have been
    /// deallocated. Failure to do so will cause undefined behavior.
    pub unsafe fn reference_range(&mut self, frame: Frame, count: usize) {
        let mut state = STATE.lock();
        let frames = state.frames_info_mut();

        let index = frame.index();
        for frame in frames[index..(index + count)].iter_mut() {
            frame.retain();
        }
    }
}

impl Default for Allocator {
    fn default() -> Self {
        Self::new()
    }
}
