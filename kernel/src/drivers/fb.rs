use crate::library::spin::Spinlock;
use embedded_graphics::{
    pixelcolor::Rgb888, prelude::*, primitives::Rectangle,
};
use spin::Lazy;

/// The kernel framebuffer object. This object provides a high-level
/// interface to the framebuffer buffer.
pub static FRAMEBUFFER: Lazy<Arc<Spinlock<Framebuffer>>> =
    Lazy::new(|| Arc::new(Spinlock::new(Framebuffer::default())));

/// A request to get the framebuffer from the bootloader.
static FB_REQUEST: limine::request::FramebufferRequest =
    limine::request::FramebufferRequest::new();

/// A framebuffer object that provides a high-level interface to a
/// framebuffer buffer. It provides a safe interface to draw pixels
/// and shapes to the framebuffer buffer.
///
/// # Limitations
/// Currently, the framebuffer only supports 32-bit framebuffers with
/// 8-bit red, green, and blue masks. This is the most common format
/// for framebuffers, but may not work on older systems when 24-bit
/// framebuffers are used to save memory.
#[derive(Default)]
pub struct Framebuffer<'a> {
    /// The width of the framebuffer, in pixels.
    pub width: usize,

    /// The height of the framebuffer, in pixels.
    pub height: usize,

    /// The number of bytes per pixel.
    pub bpp: usize,

    /// The red mask to extract the red component from a pixel.
    pub red_mask: u32,

    /// The green mask to extract the green component from a pixel.
    pub green_mask: u32,

    /// The blue mask to extract the blue component from a pixel.
    pub blue_mask: u32,

    /// The framebuffer buffer.
    pub buffer: &'a mut [u32],
}

impl Framebuffer<'_> {
    /// Create a placeholder framebuffer with no buffer and all fields
    /// zeroed. This is useful when you need a framebuffer object but
    /// don't have a valid framebuffer to initialize it with.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            buffer: &mut [],
            width: 0,
            height: 0,
            bpp: 0,
            red_mask: 0,
            green_mask: 0,
            blue_mask: 0,
        }
    }

    /// Verify if the framebuffer is valid. If the framebuffer is invalid, this
    /// probably means that the framebuffer is not initialized or doesn't
    /// exist.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !self.buffer.is_empty()
            && self.width > 0
            && self.height > 0
            && self.bpp > 0
    }

    /// Set the color of a pixel at the specified coordinates without
    /// checking the bounds.
    ///
    /// # Safety
    /// This function is unsafe because it doesn't check the bounds of
    /// the coordinates. If the coordinates are out of bounds, this
    /// function may write to an invalid memory location and the behavior
    /// is undefined.
    #[inline]
    pub unsafe fn draw_pixel_unsafe(&mut self, x: usize, y: usize, c: Color) {
        self.buffer[y * self.width + x] = self.make_pixel(c);
    }

    /// Set the color of a pixel at the specified coordinates. If the
    /// coordinates are out of bounds, this function does nothing.
    ///
    /// # Performance
    /// This function checks the bounds of the coordinates before setting
    /// the pixel color. If you know that the coordinates are within the
    /// bounds, you can call `draw_pixel_unsafe` instead to skip the
    /// bounds check. This can greatly improve performance in a hot loop.
    #[inline]
    pub fn draw_pixel(&mut self, x: usize, y: usize, c: Color) {
        if x < self.width && y < self.height {
            // SAFETY: The coordinates are checked to be within the
            // bounds of the framebuffer.
            unsafe {
                self.draw_pixel_unsafe(x, y, c);
            }
        }
    }

    /// Clear the framebuffer with the specified color.
    pub fn clear(&mut self, color: Color) {
        let color = self.make_pixel(color);
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    /// Compute a 32-bit pixel value from a color. The color components
    /// are shifted to match the framebuffer's red, green, and blue masks.
    /// The alpha component is ignored.
    ///
    /// # Performance
    /// This function is marked as `#[inline]` to hint the compiler to
    /// inline it, which should improve performance. However, you should
    /// call this function only when necessary and cache the result if
    /// you need to convert the same color multiple times.
    #[inline]
    #[must_use]
    fn make_pixel(&self, color: Color) -> u32 {
        let green_shift = self.green_mask.trailing_zeros();
        let blue_shift = self.blue_mask.trailing_zeros();
        let red_shift = self.red_mask.trailing_zeros();

        (u32::from(color.r) << red_shift) & self.red_mask
            | (u32::from(color.g) << green_shift) & self.green_mask
            | (u32::from(color.b) << blue_shift) & self.blue_mask
    }
}

impl Dimensions for Framebuffer<'_> {
    /// Get the bounding box of the framebuffer. This is a rectangle
    /// with the top-left corner at `(0, 0)` and the bottom-right corner
    /// at `(width, height)`.
    fn bounding_box(&self) -> Rectangle {
        let height = self.height as u32;
        let width = self.width as u32;
        Rectangle::new(Point::zero(), Size::new(width, height))
    }
}

impl DrawTarget for Framebuffer<'_> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    /// Draw pixels to the framebuffer. This function is called by the
    /// embedded-graphics library to draw pixels to the framebuffer.
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            self.draw_pixel(
                point.x as usize,
                point.y as usize,
                Color {
                    r: color.r(),
                    g: color.g(),
                    b: color.b(),
                },
            );
        }

        Ok(())
    }
}

impl core::fmt::Debug for Framebuffer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Framebuffer").finish()
    }
}

/// A color with 8 bits red, green, and blue components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
}

/// Setup the framebuffer by getting the framebuffer response from the
/// bootloader and create a framebuffer object from the response. If no
/// framebuffers are found in the response, this function will not initialize
/// the `FRAMEBUFFER` global variable.
pub fn setup() -> Option<()> {
    let response = FB_REQUEST.get_response()?;
    let fb = response.framebuffers().next()?;

    // Check if the framebuffer is in a supported format. For simplicity
    // and performance reasons, we only support 32-bit framebuffers with
    // 8-bit red, green, and blue masks.
    assert!(fb.bpp() == 32, "Only 32bpp framebuffers are supported");
    assert!(
        fb.green_mask_size() == 8,
        "Only 8-bit green masks are supported"
    );
    assert!(
        fb.blue_mask_size() == 8,
        "Only 8-bit blue masks are supported"
    );
    assert!(
        fb.red_mask_size() == 8,
        "Only 8-bit red masks are supported"
    );

    // Compute the size of the framebuffer buffer and create a slice
    // from the framebuffer address and its size. This will allow us
    // to write to the framebuffer buffer directly.
    let buffer = unsafe {
        core::slice::from_raw_parts_mut(
            fb.addr().cast::<u32>(),
            fb.width() as usize * fb.height() as usize,
        )
    };

    // Compute the red, green, and blue masks from the framebuffer
    let green_mask = ((1 << fb.green_mask_size()) - 1) << fb.green_mask_shift();
    let blue_mask = ((1 << fb.blue_mask_size()) - 1) << fb.blue_mask_shift();
    let red_mask = ((1 << fb.red_mask_size()) - 1) << fb.red_mask_shift();

    *FRAMEBUFFER.lock() = Framebuffer {
        height: fb.height() as usize,
        width: fb.width() as usize,
        bpp: fb.bpp() as usize / 8,
        green_mask,
        blue_mask,
        red_mask,
        buffer,
    };
    Some(())
}
