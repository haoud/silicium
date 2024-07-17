use crate::drivers::kbd::KeyboardScancodeStream;
use core::pin::Pin;
use futures::{Stream, StreamExt};
use pc_keyboard::{
    layouts, DecodedKey, EventDecoder, HandleControl, KeyState, ScancodeSet,
    ScancodeSet1,
};

/// A terminal input stream. It is an abstraction over a stream of characters
/// that allow reading lines from different sources, such as a keyboard or a
/// serial port.
pub struct TerminalInput {
    pub stream: Pin<Box<dyn Stream<Item = Result<char, ()>> + Send>>,
}

impl TerminalInput {
    /// Create a new `TerminalInput` instance from a stream of characters.
    #[must_use]
    pub fn new(
        stream: Pin<Box<dyn Stream<Item = Result<char, ()>> + Send>>,
    ) -> Self {
        Self { stream }
    }
}

impl core::fmt::Debug for TerminalInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TerminalInput").finish()
    }
}

/// A keyboard character stream. It is an abstraction over a stream of
/// characters that reads scancodes from the keyboard and converts them to
/// characters. It is higher-level than the `KeyboardScancodeStream` and
/// provide less control over the keyboard, but it is easier to use especially
/// with the `async`/`await` syntax.
pub struct KeyboardCharStream {
    /// The keyboard scancode stream. It is used to read scancodes from the
    /// keyboard and to convert them to characters.
    keyboard: KeyboardScancodeStream,

    /// The event decoder. It is used to convert scancodes to characters. It
    /// uses the AZERTY layout and maps control characters to Unicode.
    decoder: EventDecoder<layouts::Azerty>,

    /// The scancode set. It is used to keep track of the state of the keyboard
    /// and to convert scancodes to keys.
    set: ScancodeSet1,
}

impl KeyboardCharStream {
    /// Create a new `KeyboardCharStream` instance from a keyboard scancode
    /// stream. It reads scancodes from the keyboard and converts them to
    /// characters.
    #[must_use]
    pub fn new(keyboard: KeyboardScancodeStream) -> Self {
        Self {
            decoder: EventDecoder::new(
                layouts::Azerty,
                HandleControl::MapLettersToUnicode,
            ),
            set: ScancodeSet1::new(),
            keyboard,
        }
    }
}

impl Stream for KeyboardCharStream {
    type Item = Result<char, ()>;

    /// Poll the next character from the keyboard scancode stream. It reads
    /// scancodes from the keyboard and converts them to characters.
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut core::task::Context,
    ) -> core::task::Poll<Option<Self::Item>> {
        loop {
            // Advance the keyboard state and get the key from the scancode
            let scancode = match self.as_mut().keyboard.poll_next_unpin(cx) {
                core::task::Poll::Ready(scancode) => {
                    if let Some(scancode) = scancode {
                        scancode
                    } else {
                        return core::task::Poll::Ready(None);
                    }
                }
                core::task::Poll::Pending => return core::task::Poll::Pending,
            };

            // Get the key from the scancode
            if let Some(key) = self
                .as_mut()
                .set
                .advance_state(scancode)
                .expect("Failed to advance the keyboard state")
            {
                // If the key is pressed, try to decode it. If it is a
                // character, return it, otherwise, ignore it and read
                // the next scancode, hoping it will be a character.
                if key.state == KeyState::Down {
                    if let Some(DecodedKey::Unicode(c)) =
                        self.as_mut().decoder.process_keyevent(key)
                    {
                        return core::task::Poll::Ready(Some(Ok(c)));
                    };
                } else {
                    // If the key is released, we still need to process it to
                    // update the state of the keyboard in case the key is
                    // a modifier key (e.g. Shift, Control, Alt...)
                    self.as_mut().decoder.process_keyevent(key);
                }
            }
        }
    }
}
