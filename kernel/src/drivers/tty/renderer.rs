use super::Position;
use crate::{
    drivers::fb::{Color, Framebuffer},
    future,
};
use alloc::string::ToString;
use core::time::Duration;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyle, StyledDrawable},
    text::{renderer::CharacterStyle, Text},
};

/// A terminal renderer to an framebuffer. This structure is responsible
/// for rendering the text to the framebuffer. It subdivides the screen
/// into a grid of 10x20 characters and renders each character at the
/// corresponding position.
///
/// If the screen resolution is not a multiple of 10x20, the text will
/// be centered on the screen by a few pixels to avoid having the text
/// stuck to the left and top borders of the screen.
#[derive(Debug)]
pub struct TerminalRenderer {
    /// The framebuffer to render to
    framebuffer: Arc<future::Mutex<Framebuffer<'static>>>,

    /// An offset to the left border of the screen to center the text
    /// on the screen if the character grid is not matching perfectly
    /// the screen resolution.
    x_border: usize,

    /// An offset to the top border of the screen to center the text
    /// on the screen if the character grid is not matching perfectly
    /// the screen resolution.
    y_border: usize,
}

impl TerminalRenderer {
    /// Create a new terminal renderer that will render to the provided
    /// framebuffer. All the framebuffer will be used to render the text
    /// (i.e a full screen terminal)
    #[must_use]
    pub fn new(framebuffer: Arc<future::Mutex<Framebuffer<'static>>>) -> Self {
        let x_border = framebuffer.lock_blocking().width % 10 / 2;
        let y_border = framebuffer.lock_blocking().height % 20 / 2;

        Self {
            y_border,
            x_border,
            framebuffer,
        }
    }

    /// Draw a character at the specified position on the screen
    pub async fn draw_char(&self, position: Position, character: char) {
        let style = MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE);
        let point = Point {
            x: self.x_border as i32 + position.x as i32 * 10,
            y: self.y_border as i32 + position.y as i32 * 20 + 20,
        };
        Text::new(&character.to_string(), point, style)
            .draw(&mut *self.framebuffer.lock().await)
            .expect("Failed to draw character");
    }

    /// Clear the character at the specified position.
    pub async fn clear_char(&self, position: Position) {
        let mut style = MonoTextStyle::new(&FONT_10X20, Rgb888::BLACK);
        let point = Point {
            x: self.x_border as i32 + position.x as i32 * 10,
            y: self.y_border as i32 + position.y as i32 * 20 + 20,
        };
        style.set_background_color(Some(Rgb888::BLACK));
        Text::new(" ", point, style)
            .draw(&mut *self.framebuffer.lock().await)
            .expect("Failed to draw character");
    }

    /// Remove the cursor at the specified position and replace it with
    /// the specified character.
    pub async fn clear_cursor(&self, cursor: Position, character: char) {
        self.clear_char(cursor).await;
        self.draw_char(cursor, character).await;
    }

    /// Redraw the cursor at the specified position. This function does not
    /// clear the previous cursor position (if any)!: this is the caller's
    /// responsibility to do so if needed.
    pub async fn redraw_cursor_at(&self, cursor: Position) {
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
                &mut *self.framebuffer.lock().await,
            )
            .expect("Failed to draw cursor");
    }

    /// Clear the terminal screen by filling it with black.
    pub async fn clear(&self) {
        self.framebuffer.lock().await.clear(Color::BLACK);
    }
}

#[derive(Debug)]
pub struct BlinkingCursor {
    /// The renderer to use to draw the cursor
    pub renderer: Arc<TerminalRenderer>,

    /// The position of the cursor on the screen. The position is in the
    /// terminal grid, not in the framebuffer grid.
    pub position: Position,

    /// The speed at which the cursor should blink
    pub speed: Duration,

    /// The character located at the cursor position, used to redraw the
    /// cursor when it blinks.
    pub character: char,
}

/// Blink the cursor at the specified position. This function will draw
/// the cursor at the specified position and make it blink at the specified
/// speed, indefinitely. To stop the cursor blinking, the caller must cancel
/// the task that runs this function.
pub async fn blink_cursor(cursor: BlinkingCursor) {
    let renderer = &cursor.renderer;
    let char = cursor.character;
    let pos = cursor.position;
    loop {
        renderer.redraw_cursor_at(pos).await;
        future::sleep::sleep(cursor.speed).await;
        renderer.clear_cursor(pos, char).await;
        future::sleep::sleep(cursor.speed).await;
    }
}
