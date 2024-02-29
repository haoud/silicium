use core::ops::{BitAnd, BitOr};

use super::opcode;

pub trait IO {
    /// Write a value to a port.
    ///
    /// # Safety
    /// This function is unsafe because writing to a port can have side effects, including
    /// causing the hardware to do something unexpected and possibly violating memory safety.
    unsafe fn write(port: u16, value: Self);

    /// Read a value from a port.
    ///
    /// # Safety
    /// This function is unsafe because reading from a port can have side effects, including
    /// causing the hardware to do something unexpected and possibly violating memory safety.
    unsafe fn read(port: u16) -> Self;

    /// Write a value to a port, then pause for a short time. This is useful for
    /// writing to ports that require a short delay after writing in order to let
    /// enough time pass for the hardware to process the write.
    ///
    /// # Safety
    /// This function is unsafe because writing to a port can have side effects, including
    /// causing the hardware to do something unexpected and possibly violating memory safety.
    unsafe fn write_and_pause(port: u16, value: Self)
    where
        Self: Sized,
    {
        Self::write(port, value);
        pause();
    }
}

impl IO for u8 {
    unsafe fn write(port: u16, value: u8) {
        opcode::outb(port, value);
    }

    unsafe fn read(port: u16) -> u8 {
        opcode::inb(port)
    }
}

impl IO for u16 {
    unsafe fn write(port: u16, value: u16) {
        opcode::outw(port, value);
    }

    unsafe fn read(port: u16) -> u16 {
        opcode::inw(port)
    }
}

impl IO for u32 {
    unsafe fn write(port: u16, value: u32) {
        opcode::outd(port, value);
    }

    unsafe fn read(port: u16) -> u32 {
        opcode::ind(port)
    }
}

/// Represents a port that can be read from and written to. This is a wrapper around a port number
/// and a type that implements the `IO` trait (currently `u8`, `u16`, or `u32`).
#[derive(Debug)]
pub struct Port<T> {
    phantom: core::marker::PhantomData<T>,
    port: u16,
}

impl<T: IO> Port<T> {
    /// Create a new port. This function is safe because it does not access any hardware, it
    /// simply encapsulates a port number and a type that implements the `IO` trait.
    #[must_use]
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: core::marker::PhantomData,
        }
    }

    /// Write a value to the port, then pause for a short time. This is useful for
    /// writing to ports that require a short delay after writing in order to let
    /// enough time pass for the hardware to process the write.
    ///
    /// # Safety
    /// This function is unsafe because writing to a port can have side effects, including
    /// causing the hardware to do something unexpected and possibly violating memory safety.
    pub unsafe fn write_and_pause(&self, value: T) {
        T::write_and_pause(self.port, value);
    }

    /// Write a value to the port.
    ///
    /// # Safety
    /// This function is unsafe because writing to a port can have side effects, including
    /// causing the hardware to do something unexpected and possibly violating memory safety.
    pub unsafe fn write(&self, value: T) {
        T::write(self.port, value);
    }

    /// Read a value from the port.
    ///
    /// # Safety
    /// This function is unsafe because reading from a port can have side effects, including
    /// causing the hardware to do something unexpected and possibly violating memory safety.
    #[must_use]
    pub unsafe fn read(&self) -> T {
        T::read(self.port)
    }
}

impl<T: IO + BitOr<T, Output = T>> Port<T> {
    /// Set a bit in the port.
    ///
    /// # Safety
    /// This function is unsafe because writing to a port can have side effects, including
    /// causing the hardware to do something unexpected and possibly violating memory safety.
    pub unsafe fn set_bits(&self, value: T) {
        self.write(self.read() | value);
    }
}

impl<T: IO + BitAnd<T, Output = T>> Port<T> {
    /// Clear a bit in the port.
    ///
    /// # Safety
    /// This function is unsafe because writing to a port can have side effects, including
    /// causing the hardware to do something unexpected and possibly violating memory safety.
    pub unsafe fn clear_bits(&self, value: T) {
        self.write(self.read() & value);
    }
}

/// Pause for a short time. This is useful for writing to ports that require a short delay after
/// writing in order to let enough time pass for the hardware to process the write.
///
/// # Safety
/// Currently this function is implemented by writing to port 0x80, which is used by Linux,
/// but it may be fragile as it assumes that the port 0x80 is not used by the hardware. This is
/// why this function is marked as unsafe, through it should be safe to use in practice.
pub unsafe fn pause() {
    opcode::outb(0x80, 0);
}
