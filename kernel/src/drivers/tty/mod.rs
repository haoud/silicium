use super::fb::{Color, Framebuffer};
use alloc::string::ToString;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyle, StyledDrawable},
    text::{renderer::CharacterStyle, Text},
};

/// A virtual terminal. This structure represents a terminal that can be
/// written to and flushed to a framebuffer. It is a simple implementation
/// of a terminal and only contains the necessary functions to emulate the
/// terminal behavior. The rendering is done using the `TerminalRenderer`
/// structure.
#[derive(Debug)]
pub struct VirtualTerminal<'a> {
    /// The character buffer. It represents the characters
    /// visible on the screen.
    character: Vec<char>,

    /// The framebuffer to render to
    renderer: TerminalRenderer<'a>,

    /// The position of the cursor in the framebuffer
    drawn_cursor: Position,

    /// The cursor position in the grid
    cursor: Position,

    /// The number of characters per column
    height: usize,

    /// The number of characters per row
    width: usize,
}

impl<'a> VirtualTerminal<'a> {
    /// Create a new virtual terminal that will use the provided framebuffer
    /// to render the text.
    #[must_use]
    pub fn new(framebuffer: Framebuffer<'a>) -> Self {
        let height = framebuffer.height / 20;
        let width = framebuffer.width / 10;
        log::trace!("Creating virtual terminal with size {}x{}", width, height);
        Self {
            character: vec![' '; height * width],
            renderer: TerminalRenderer::new(framebuffer),
            drawn_cursor: Position { x: 0, y: 0 },
            cursor: Position { x: 0, y: 0 },
            height,
            width,
        }
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
                self.character[position] = character;
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
            self.character.copy_within(self.width.., 0);
            self.character.truncate(self.width * (self.height - 1));
            self.character.extend((0..self.width).map(|_| ' '));
            self.cursor.y = self.height - 1;
            self.flush();
        }
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
        for (i, character) in self.character.iter().enumerate() {
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
        let char = self.character[offset];

        self.renderer.clear_cursor(self.drawn_cursor, char);
        self.renderer.redraw_cursor_at(self.cursor);
        self.drawn_cursor = self.cursor;
    }
}

/// A terminal renderer to an framebuffer. This structure is responsible
/// for rendering the text to the framebuffer. It subdivides the screen
/// into a grid of 10x20 characters and renders each character at the
/// corresponding position.
///
/// If the screen resolution is not a multiple of 10x20, the text will
/// be centered on the screen by a few pixels to avoid having the text
/// stuck to the left and top borders of the screen.
#[derive(Debug)]
pub struct TerminalRenderer<'a> {
    /// The framebuffer to render to
    framebuffer: Framebuffer<'a>,

    /// An offset to the left border of the screen to center the text
    /// on the screen if the character grid is not matching perfectly
    /// the screen resolution.
    x_border: usize,

    /// An offset to the top border of the screen to center the text
    /// on the screen if the character grid is not matching perfectly
    /// the screen resolution.
    y_border: usize,
}

impl<'a> TerminalRenderer<'a> {
    /// Create a new terminal renderer that will render to the provided
    /// framebuffer. All the framebuffer will be used to render the text
    /// (i.e a full screen terminal)
    #[must_use]
    pub fn new(framebuffer: Framebuffer<'a>) -> Self {
        Self {
            y_border: (framebuffer.height % 20) / 2,
            x_border: (framebuffer.width % 10) / 2,
            framebuffer,
        }
    }

    /// Draw a character at the specified position on the screen
    pub fn draw_char(&mut self, position: Position, character: char) {
        let style = MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE);
        let point = Point {
            x: self.x_border as i32 + position.x as i32 * 10,
            y: self.y_border as i32 + position.y as i32 * 20 + 20,
        };
        Text::new(&character.to_string(), point, style)
            .draw(&mut self.framebuffer)
            .expect("Failed to draw character");
    }

    /// Clear the character at the specified position.
    pub fn clear_char(&mut self, position: Position) {
        let mut style = MonoTextStyle::new(&FONT_10X20, Rgb888::BLACK);
        let point = Point {
            x: self.x_border as i32 + position.x as i32 * 10,
            y: self.y_border as i32 + position.y as i32 * 20 + 20,
        };
        style.set_background_color(Some(Rgb888::BLACK));
        Text::new(" ", point, style)
            .draw(&mut self.framebuffer)
            .expect("Failed to draw character");
    }

    /// Remove the cursor at the specified position and replace it with
    /// the specified character.
    pub fn clear_cursor(&mut self, cursor: Position, character: char) {
        self.clear_char(cursor);
        self.draw_char(cursor, character);
    }

    /// Redraw the cursor at the specified position. This function does not
    /// clear the previous cursor position (if any)!: this is the caller's
    /// responsibility to do so if needed.
    pub fn redraw_cursor_at(&mut self, cursor: Position) {
        let start = Point {
            x: self.x_border as i32 + cursor.x as i32 * 10,
            y: self.y_border as i32 + cursor.y as i32 * 20 + 20,
        };
        let end = Point {
            x: self.x_border as i32 + cursor.x as i32 * 10 + 9,
            y: self.y_border as i32 + cursor.y as i32 * 20 + 20,
        };

        Line::new(start, end)
            .draw_styled(
                &PrimitiveStyle::with_stroke(Rgb888::WHITE, 2),
                &mut self.framebuffer,
            )
            .expect("Failed to draw cursor");

        // TODO: Async task for blinking cursor
    }

    /// Clear the terminal screen by filling it with black.
    pub fn clear(&mut self) {
        self.framebuffer.clear(Color::BLACK);
    }
}

/// Represents a 2D position on the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
