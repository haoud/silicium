use crate::{
    drivers::fb::Framebuffer, future::executor, library::spin::Spinlock,
};
use async_task::Task;
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
    characters: Vec<char>,

    /// The framebuffer to render to
    renderer: Arc<TerminalRenderer>,

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
    pub fn new(
        framebuffer: Arc<Spinlock<Framebuffer<'static>>>,
        input: TerminalInput,
    ) -> Self {
        let height = framebuffer.lock().height / 20;
        let width = framebuffer.lock().width / 10;
        log::trace!("Creating virtual terminal with size {}x{}", width, height);

        let renderer = Arc::new(TerminalRenderer::new(framebuffer));

        // Create a new blinking task with the terminal renderer at the cursor
        // position with the character to blink and the blinking speed and
        // schedule the task to be run.
        let (runnable, blinking) =
            executor::spawn(renderer::blink_cursor(BlinkingCursor {
                renderer: Arc::clone(&renderer),
                position: Position { x: 0, y: 0 },
                speed: Duration::from_millis(500),
                character: ' ',
            }));
        runnable.schedule();

        Self {
            characters: vec![' '; height * width],
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
                        self.erase_char();
                    }
                }
                Some(Ok(char)) => {
                    self.write_char(char);
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
    pub fn write(&mut self, character: char) {
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
                self.renderer.draw_char(
                    Position {
                        x: self.cursor.x,
                        y: self.cursor.y,
                    },
                    character,
                );

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
            self.characters.extend((0..self.width).map(|_| ' '));
            self.cursor.y = self.height - 1;
            self.flush();
        }
    }

    /// Erase the character just before the cursor and update the cursor
    /// position accordingly. If the cursor is at the beginning of a line,
    /// the cursor will be moved to the end of the previous line. If the
    /// cursor is at the beginning of the first line, nothing will happen.
    pub fn erase_char(&mut self) {
        if self.cursor.x == 0 {
            if self.cursor.y > 0 {
                self.cursor.y -= 1;
                self.cursor.x = self.width - 1;
            }
        } else {
            self.cursor.x -= 1;
        }

        let position = self.cursor.y * self.width + self.cursor.x;
        self.characters[position] = ' ';
        self.renderer.clear_char(self.cursor);
        self.update_cursor();
    }

    /// Write a string to the terminal and update the cursor position
    /// accordingly.
    pub fn write_str(&mut self, string: &str) {
        for character in string.chars() {
            self.write(character);
        }
        self.update_cursor();
    }

    /// Write a character to the terminal and update the cursor position
    /// accordingly.
    pub fn write_char(&mut self, character: char) {
        self.write(character);
        self.update_cursor();
    }

    /// Flush the terminal and redraw the screen
    pub fn flush(&mut self) {
        self.renderer.clear();
        self.renderer.redraw_cursor_at(self.drawn_cursor);
        for (i, character) in self.characters.iter().enumerate() {
            self.renderer.draw_char(
                Position {
                    x: i % self.width,
                    y: i / self.width,
                },
                *character,
            );
        }
    }

    /// Update the cursor position into the framebuffer. The cursor currently
    /// drawn on the screen will be erased and redrawn at the new position.
    fn update_cursor(&mut self) {
        let offset = self.drawn_cursor.y * self.width + self.drawn_cursor.x;
        let char = self.characters[offset];

        self.renderer.clear_cursor(self.drawn_cursor, char);
        self.renderer.redraw_cursor_at(self.cursor);
        self.drawn_cursor = self.cursor;
        self.create_blinking_task();
    }

    /// Cancel the current blinking task and create a new one at the cursor
    /// position. The new blinking task will be scheduled in the background
    /// when the old one has been canceled, avoiding having multiple blinking
    /// tasks running at the same time.
    fn create_blinking_task(&mut self) {
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

        // Replace the old blinking task with the new one. The new task will
        // not be scheduled immediately, but only after the old one has been
        // canceled to avoid having multiple blinking tasks running at the
        // same time. We do this in the background to avoid blocking the caller.
        let outdated = core::mem::replace(&mut self.blinking, blinking);
        executor::schedule_detached(async move {
            outdated.cancel().await;
            // TODO: Clean the old cursor position
            runnable.schedule();
        });
    }
}

impl core::fmt::Write for VirtualTerminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

/// Represents a 2D position on the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
