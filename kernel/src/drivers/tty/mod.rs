use crate::{
    drivers::fb::Framebuffer,
    future::{self, executor},
};
use async_task::Task;
use bitflags::bitflags;
use core::time::Duration;
use futures::StreamExt;
use input::TerminalInput;
use renderer::{BlinkingCursor, TerminalRenderer};

pub mod input;
pub mod renderer;

/// A virtual terminal. This structure represents a terminal that can be
/// written to and flushed to a framebuffer. It is a simple implementation
/// of a terminal and only contains the necessary functions to emulate the
/// terminal behavior. The rendering is done using the `TerminalRenderer`
/// structure.
#[derive(Debug)]
pub struct VirtualTerminal {
    /// The character buffer. It represents the characters
    /// visible on the screen.
    characters: Vec<Character>,

    /// The framebuffer to render to
    renderer: Arc<TerminalRenderer>,

    /// The foreground color of the text
    char_foreground: AnsiColor16,

    /// The background color of the text
    char_background: AnsiColor16,

    /// The blinking cursor task
    blinking: Task<()>,

    /// The terminal input
    input: TerminalInput,

    /// The position of the cursor in the framebuffer
    drawn_cursor: Position,

    /// The cursor position in the grid
    cursor: Position,

    /// The number of characters per column
    height: usize,

    /// The number of characters per row
    width: usize,
}

impl VirtualTerminal {
    /// Create a new virtual terminal that will use the provided framebuffer
    /// to render the text.
    #[must_use]
    pub async fn new(
        framebuffer: Arc<future::Mutex<Framebuffer<'static>>>,
        input: TerminalInput,
    ) -> Self {
        let height = framebuffer.lock().await.height / 20;
        let width = framebuffer.lock().await.width / 10;
        log::trace!("Creating virtual terminal with size {}x{}", width, height);

        let renderer = Arc::new(TerminalRenderer::new(framebuffer));

        // The default style of the characters
        let style = CharStyle {
            foreground: AnsiColor16::White,
            background: AnsiColor16::Black,
            effects: CharEffects::empty(),
        };

        // Create a new blinking task with the terminal renderer at the cursor
        // position with the character to blink and the blinking speed and
        // schedule the task to be run.
        let (runnable, blinking) =
            executor::spawn(renderer::blink_cursor(BlinkingCursor {
                renderer: Arc::clone(&renderer),
                position: Position { x: 0, y: 0 },
                speed: Duration::from_millis(500),
                character: Character::space(style),
            }));
        runnable.schedule();

        Self {
            characters: vec![Character::space(style); height * width],
            char_foreground: AnsiColor16::White,
            char_background: AnsiColor16::Black,
            drawn_cursor: Position { x: 0, y: 0 },
            cursor: Position { x: 0, y: 0 },
            blinking,
            renderer,
            height,
            width,
            input,
        }
    }

    /// Read a line from the input stream.
    pub async fn readline(&mut self) -> String {
        let mut line = String::new();
        loop {
            match self.input.stream.as_mut().next().await {
                Some(Ok('\x08')) => {
                    // Only erase if there is a character to erase in the
                    // line to avoid cursor glitches
                    if line.pop().is_some() {
                        self.erase_char().await;
                    }
                }
                Some(Ok(char)) => {
                    self.write_char(char).await;
                    if char == '\n' {
                        break;
                    } else {
                        line.push(char);
                    }
                }
                None => todo!("Handle EOF"),
                _ => break,
            }
        }
        line
    }

    /// Write a character to the terminal. This function will _NOT_ update
    /// the cursor position, it is up to the caller to do so if needed.
    pub async fn write(&mut self, character: char) {
        match character {
            '\n' => {
                self.cursor.y += 1;
                self.cursor.x = 0;
            }
            '\r' => {
                self.cursor.x = 0;
            }
            _ => {
                // Draw the character on the screen
                let character = Character {
                    value: character,
                    style: CharStyle {
                        foreground: self.char_foreground,
                        background: self.char_background,
                        effects: CharEffects::empty(),
                    },
                };

                let position = Position {
                    x: self.cursor.x,
                    y: self.cursor.y,
                };

                self.renderer.draw_char(position, character).await;

                // Update the character buffer
                let position = self.cursor.y * self.width + self.cursor.x;
                self.characters[position] = character;
                self.cursor.x += 1;
            }
        }

        // If the cursor goes out of the screen, we need to move it to
        // the next line
        if self.cursor.x >= self.width {
            self.cursor.y += 1;
            self.cursor.x = 0;
        }

        // If we go out of the screen, we need to scroll the screen
        // to make the cursor visible. We must move all the characters
        // one line up and remove the last line.
        if self.cursor.y >= self.height {
            self.characters.copy_within(self.width.., 0);
            self.characters.truncate(self.width * (self.height - 1));
            self.characters.extend((0..self.width).map(|_| {
                Character::space(CharStyle {
                    foreground: self.char_foreground,
                    background: self.char_background,
                    effects: CharEffects::empty(),
                })
            }));
            self.cursor.y = self.height - 1;
            self.flush().await;
        }
    }

    /// Erase the character just before the cursor and update the cursor
    /// position accordingly. If the cursor is at the beginning of a line,
    /// the cursor will be moved to the end of the previous line. If the
    /// cursor is at the beginning of the first line, nothing will happen.
    pub async fn erase_char(&mut self) {
        if self.cursor.x == 0 {
            if self.cursor.y > 0 {
                self.cursor.y -= 1;
                self.cursor.x = self.width - 1;
            }
        } else {
            self.cursor.x -= 1;
        }

        let position = self.cursor.y * self.width + self.cursor.x;
        let character = Character::space(CharStyle {
            foreground: self.char_foreground,
            background: self.char_background,
            effects: CharEffects::empty(),
        });

        self.characters[position] = character;
        self.renderer.draw_char(self.cursor, character).await;
        self.update_cursor().await;
    }

    /// Write a string to the terminal and update the cursor position
    /// accordingly.
    pub async fn write_str(&mut self, string: &str) {
        for character in string.chars() {
            self.write(character).await;
        }
        self.update_cursor().await;
    }

    /// Write a character to the terminal and update the cursor position
    /// accordingly.
    pub async fn write_char(&mut self, character: char) {
        self.write(character).await;
        self.update_cursor().await;
    }

    /// Flush the terminal and redraw the screen
    pub async fn flush(&mut self) {
        self.renderer.clear().await;
        self.renderer.redraw_cursor_at(self.drawn_cursor).await;
        for (i, character) in self.characters.iter().enumerate() {
            self.renderer
                .draw_char(
                    Position {
                        x: i % self.width,
                        y: i / self.width,
                    },
                    *character,
                )
                .await;
        }
    }

    /// Update the cursor position into the framebuffer. The cursor currently
    /// drawn on the screen will be erased and redrawn at the new position.
    async fn update_cursor(&mut self) {
        // TODO: Only needed if the cursor blinking is disabled
        // let offset = self.drawn_cursor.y * self.width + self.drawn_cursor.x;
        // let char = self.characters[offset];
        //self.renderer.clear_cursor(self.drawn_cursor, char).await;
        //self.renderer.redraw_cursor_at(self.cursor).await;

        let old_cursor = self.drawn_cursor;
        let old_offset = old_cursor.y * self.width + old_cursor.x;
        let old_char = self.characters[old_offset];
        self.drawn_cursor = self.cursor;
        self.create_blinking_task(old_cursor, old_char);
    }

    /// Cancel the current blinking task and create a new one at the cursor
    /// position. The new blinking task will be scheduled in the background
    /// when the old one has been canceled, avoiding having multiple blinking
    /// tasks running at the same time.
    fn create_blinking_task(
        &mut self,
        old_cursor: Position,
        old_char: Character,
    ) {
        let offset = self.cursor.y * self.width + self.cursor.x;
        let char = self.characters[offset];

        // Create a new blinking task with the terminal renderer at the cursor
        // position with the character to blink and the blinking speed.
        let (runnable, blinking) =
            executor::spawn(renderer::blink_cursor(BlinkingCursor {
                renderer: Arc::clone(&self.renderer),
                position: self.cursor,
                speed: Duration::from_millis(500),
                character: char,
            }));

        // Replace the old blinking task with the new one.
        let outdated = core::mem::replace(&mut self.blinking, blinking);
        executor::block_on(async {
            outdated.cancel().await;
            self.renderer.clear_cursor(old_cursor, old_char).await;
            runnable.schedule();
        });
    }
}

impl core::fmt::Write for VirtualTerminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        executor::block_on(self.write_str(s));
        Ok(())
    }
}

/// Represents a 16-color ANSI color. The colors are the same as the ones
/// used in the traditional VGA text mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AnsiColor16 {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    Gray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CharEffects : u8 {
        // No effect implemented yet
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Character {
    /// The character to display
    pub value: char,

    /// The style of the character
    pub style: CharStyle,
}

impl Character {
    /// Create a new space character with the given style. This is useful
    /// for displaying empty spaces on the screen or for an placeholder
    /// character.
    #[must_use]
    pub fn space(style: CharStyle) -> Self {
        Self { value: ' ', style }
    }
}

/// Represents the style of a character on the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharStyle {
    /// The foreground color of the character
    pub foreground: AnsiColor16,

    /// The background color of the character
    pub background: AnsiColor16,

    /// The effects applied to the character; e.g. bold, italic, underline...
    /// Each effect is represented by a bit in the `CharEffect` bitfield.
    pub effects: CharEffects,
}

/// Represents a 2D position on the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
