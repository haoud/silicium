use super::{AnsiColor16, Character, Position};
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
    text::Text,
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
    pub async fn draw_char(&self, position: Position, char: Character) {
        let mut style = MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE);
        style.background_color = Some(map_ansi_color(char.style.background));
        style.text_color = Some(map_ansi_color(char.style.foreground));

        let point = Point {
            x: self.x_border as i32 + position.x as i32 * 10,
            y: self.y_border as i32 + position.y as i32 * 20 + 20,
        };

        Text::new(&char.value.to_string(), point, style)
            .draw(&mut *self.framebuffer.lock().await)
            .expect("Failed to draw character");
    }

    /// Remove the cursor at the specified position and replace it with
    /// the specified character.
    pub async fn clear_cursor(&self, cursor: Position, char: Character) {
        let space = Character::space(char.style);
        self.draw_char(cursor, space).await;
        self.draw_char(cursor, char).await;
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

/// A blinking cursor that can be displayed on the screen. This structure
/// contains everything needed to display a blinking cursor at a specific
/// position on the screen:
/// - The renderer to use to draw the cursor
/// - The position of the cursor on the screen
/// - The speed at which the cursor should blink
/// - The character located at the cursor position, used remove the cursor
///   when it blinks.
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
    pub character: Character,
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

/// Translate an ANSI color to a pixel color
pub fn map_ansi_color(ansi: AnsiColor16) -> Rgb888 {
    match ansi {
        AnsiColor16::Black => Rgb888::BLACK,
        AnsiColor16::Blue => Rgb888::BLUE,
        AnsiColor16::Green => Rgb888::GREEN,
        AnsiColor16::Cyan => Rgb888::CYAN,
        AnsiColor16::Red => Rgb888::CSS_DARK_RED,
        AnsiColor16::Magenta => Rgb888::CSS_DARK_MAGENTA,
        AnsiColor16::Brown => Rgb888::CSS_BROWN,
        AnsiColor16::LightGray => Rgb888::CSS_LIGHT_GRAY,
        AnsiColor16::Gray => Rgb888::CSS_GRAY,
        AnsiColor16::LightBlue => Rgb888::CSS_LIGHT_BLUE,
        AnsiColor16::LightGreen => Rgb888::CSS_LIGHT_GREEN,
        AnsiColor16::LightCyan => Rgb888::CSS_LIGHT_CYAN,
        AnsiColor16::LightRed => Rgb888::RED,
        AnsiColor16::LightMagenta => Rgb888::CSS_MAGENTA,
        AnsiColor16::Yellow => Rgb888::YELLOW,
        AnsiColor16::White => Rgb888::WHITE,
    }
}
