use crate::opcode;

/// The number of iterations to wait for the sending or receiving buffer to be ready before
/// timing out. This number is chosen to be large enough to avoid timing out in most cases, but
/// small enough to avoid waiting for too long in case of a faulty serial ports or an empty
/// receiving buffer (which can happen if no data is being sent to the port by the other device).
pub const TIMEOUT_ITER: u16 = 16384;

/// Represents a serial port. It should be the same on all `x86_64` systems, since the `x86_64`
/// architecture try to keep compatibility with the original IBM PC. However, it's not guaranteed
/// that the serial ports are present on all systems.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Port {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

impl From<Port> for u16 {
    fn from(port: Port) -> u16 {
        port as u16
    }
}

/// Represents a serial port object. It it used to safely interact with a serial port.
#[derive(Debug, Clone)]
pub struct Serial {
    port: Port,
}

impl Serial {
    /// Creates a new serial port object without initializing it.
    ///
    /// # Safety
    /// This function can cause memory unsafety if the port is not initialized before being used or
    /// if the port is not present on the system and used by another device.
    #[must_use]
    pub const unsafe fn uninitialized(port: Port) -> Self {
        Self { port }
    }

    /// Creates a new serial port object and initializes it. If the port is not present on the system
    /// or is faulty, this function returns `None`.
    #[must_use]
    pub fn new(port: Port) -> Option<Self> {
        // SAFETY: This should be safe because we use standard I/O ports to initialize
        // the serial device. If the serial device is not present, the initialization
        // will gracefully fail and return None.
        unsafe {
            // Disable interrupts
            opcode::outb(u16::from(port) + 1, 0x00);

            // Enable DLAB (set baud rate divisor)
            opcode::outb(u16::from(port) + 3, 0x80);

            // Set divisor to 3 (lo byte) 38400 baud
            opcode::outb(u16::from(port), 0x03);
            opcode::outb(u16::from(port) + 1, 0x00);

            // Set 8 bits, no parity, one stop bit
            opcode::outb(u16::from(port) + 3, 0x03);

            // Enable FIFO, clear them, with 14-byte threshold
            opcode::outb(u16::from(port) + 2, 0xC7);

            // IRQs enabled, RTS/DSR set
            opcode::outb(u16::from(port) + 4, 0x0B);

            // Set loopback mode (test the serial chip) and send a byte
            opcode::outb(u16::from(port) + 4, 0x1E);
            opcode::outb(u16::from(port), 0xAE);

            // If the byte is not echoed, the serial port is not present
            // or not functioning, and we should return None.
            if opcode::inb(u16::from(port)) != 0xAE {
                return None;
            }

            // Disable loopback mode, set the port to normal operation mode
            opcode::outb(u16::from(port) + 4, 0x0F);
        };

        Some(Self { port })
    }

    /// Sends a byte to the serial port.
    ///
    /// # Errors
    /// If the operation times out, this function returns `Err(SendError::Timeout)`.
    pub fn send(&self, byte: u8) -> Result<(), SendError> {
        for _ in 0..TIMEOUT_ITER {
            // SAFETY: This is safe because we checked in the `new` function that the port
            // is avaible and functioning. Readed or writting to a serial port should not
            // break memory safety.
            unsafe {
                if opcode::inb(u16::from(self.port) + 5) & 0x20 != 0 {
                    opcode::outb(u16::from(self.port), byte);
                    return Ok(());
                }
            }
        }

        Err(SendError::Timeout)
    }

    /// Receives a byte from the serial port.
    ///
    /// # Errors
    /// If the operation times out, this function returns `Err(ReceiveError::Timeout)`.
    pub fn receive(&self) -> Result<u8, ReceiveError> {
        for _ in 0..TIMEOUT_ITER {
            // SAFETY: This is safe because we checked in the `new` function that the port
            // is avaible and functioning. Readed or writting to a serial port should not
            // break memory safety.
            unsafe {
                if opcode::inb(u16::from(self.port) + 5) & 0x01 != 0 {
                    return Ok(opcode::inb(u16::from(self.port)));
                }
            }
        }

        Err(ReceiveError::Timeout)
    }
}

impl core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            _ = self.send(byte);
        }
        Ok(())
    }
}

/// Represents an error that can occur when sending a byte to a serial port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SendError {
    /// The operation timed out and the sending buffer is still full.
    Timeout,
}

/// Represents an error that can occur when receiving a byte from a serial port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReceiveError {
    /// The operation timed out and the receiving buffer is still empty.
    Timeout,
}
