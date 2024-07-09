use crate::library::spin::Spinlock;

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

impl core::fmt::Debug for Framebuffer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Framebuffer")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("bpp", &self.bpp)
            .finish()
    }
}

pub struct Pixel<'a> {
    framebuffer: &'a mut Framebuffer<'a>,
    x: usize,
    y: usize,
}

impl<'a> Pixel<'a> {
    /// Create a new pixel object from a framebuffer and coordinates.
    ///
    /// # Panics
    /// This function panics if the coordinates are out of bounds.
    #[must_use]
    pub fn get(
        framebuffer: &'a mut Framebuffer<'a>,
        x: usize,
        y: usize,
    ) -> Self {
        assert!(y < framebuffer.height, "y coordinate out of bounds");
        assert!(x < framebuffer.width, "x coordinate out of bounds");
        Pixel { framebuffer, x, y }
    }

    /// Get the offset of the pixel in the framebuffer buffer.
    #[must_use]
    pub fn offset(&self) -> usize {
        self.y * self.framebuffer.width + self.x
    }

    /// Get the red component of the pixel.
    #[must_use]
    pub fn red(&self) -> u8 {
        ((self.data() & self.framebuffer.red_mask)
            >> self.framebuffer.red_mask.trailing_zeros()) as u8
    }

    /// Get the green component of the pixel.
    #[must_use]
    pub fn green(&self) -> u8 {
        ((self.data() & self.framebuffer.green_mask)
            >> self.framebuffer.green_mask.trailing_zeros()) as u8
    }

    /// Get the blue component of the pixel.
    #[must_use]
    pub fn blue(&self) -> u8 {
        ((self.data() & self.framebuffer.blue_mask)
            >> self.framebuffer.blue_mask.trailing_zeros()) as u8
    }

    /// Set the red component of the pixel.
    pub fn set_red(&mut self, value: u8) {
        let shift = self.framebuffer.red_mask.trailing_zeros();
        self.set(
            self.data() & !self.framebuffer.red_mask
                | (u32::from(value) << shift & self.framebuffer.red_mask),
        );
    }

    /// Set the green component of the pixel.
    pub fn set_green(&mut self, value: u8) {
        let shift = self.framebuffer.green_mask.trailing_zeros();
        self.set(
            self.data() & !self.framebuffer.green_mask
                | (u32::from(value) << shift & self.framebuffer.green_mask),
        );
    }

    /// Set the blue component of the pixel.
    pub fn set_blue(&mut self, value: u8) {
        let shift = self.framebuffer.blue_mask.trailing_zeros();
        self.set(
            self.data() & !self.framebuffer.blue_mask
                | (u32::from(value) << shift & self.framebuffer.blue_mask),
        );
    }

    /// Set the color of the pixel.
    pub fn set_color(&mut self, color: Color) {
        self.set(self.framebuffer.make_pixel(color));
    }

    /// Set the raw 32-bit data of the pixel.
    pub fn set(&mut self, data: u32) {
        self.framebuffer.buffer[self.offset()] = data;
    }

    /// Get the raw 32-bit data of the pixel.
    #[must_use]
    pub fn data(&self) -> u32 {
        self.framebuffer.buffer[self.offset()]
    }
}

/// A color with 8 bits red, green, and blue components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

static FB_REQUEST: limine::request::FramebufferRequest =
    limine::request::FramebufferRequest::new();

static FRAMEBUFFER: Spinlock<Framebuffer> = Spinlock::new(Framebuffer::none());

pub fn setup() {
    let response = FB_REQUEST
        .get_response()
        .expect("Failed to get framebuffer response");

    let fb = response
        .framebuffers()
        .next()
        .expect("No framebuffers found");

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

    let green_mask = ((1 << fb.green_mask_size()) - 1) << fb.green_mask_shift();
    let blue_mask = ((1 << fb.blue_mask_size()) - 1) << fb.blue_mask_shift();
    let red_mask = ((1 << fb.red_mask_size()) - 1) << fb.red_mask_shift();

    let mut framebuffer = Framebuffer {
        height: fb.height() as usize,
        width: fb.width() as usize,
        bpp: fb.bpp() as usize / 8,
        green_mask,
        blue_mask,
        red_mask,
        buffer,
    };

    framebuffer.clear(Color {
        r: 32,
        g: 144,
        b: 255,
    });
    *FRAMEBUFFER.lock() = framebuffer;
}
