//
//! Defines [`FontBitmapWord`].
//

#[cfg(all(feature = "draw", feature = "image"))]
use crate::{CanvasRaster, CanvasRasterExt};
use crate::{CharIter, FontBitmapPixel, FontBitmapPixelIter, Position2};
use crate::{Debug, FmtResult, Formatter, format_buf, is, unwrap, whilst};

#[doc = crate::_tags!(font)]
/// A fixed-size bitmap font packed into glyph words.
#[doc = crate::_doc_meta!{
    location("media/font", struct FontBitmapWord),
    #[cfg(target_pointer_width = "32")]
    test_size_of(FontBitmapWord<()> = 28|224),
    #[cfg(target_pointer_width = "64")]
    test_size_of(FontBitmapWord<()> = 48|384),
}]
/// Glyph bits are stored row-major from the least-significant bit:
/// left to right, then top to bottom.
///
/// `baseline` is a zero-based glyph row. Drawing at `y` places that row at `y`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FontBitmapWord<'glyphs, T> {
    glyphs: &'glyphs [T],
    first_glyph: char,
    extra_glyphs: &'glyphs [(char, T)],
    width: u8,
    height: u8,
    baseline: u8,
    advance_x: u8,
    advance_y: u8,
}
impl<T> Debug for FontBitmapWord<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
        let mut buf = [0u8; 128];
        let name = format_buf![&mut buf, "FontBitmapWord<{}>", stringify!(T)].unwrap();
        f.debug_struct(name)
            .field("glyphs", &self.glyphs.len())
            .field("first_glyph", &self.first_glyph)
            .field("extra_glyphs", &self.extra_glyphs.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("baseline", &self.baseline)
            .field("advance_x", &self.advance_x)
            .field("advance_y", &self.advance_y)
            .finish()
    }
}

#[rustfmt::skip]
impl<'glyphs, T> FontBitmapWord<'glyphs, T> {
    /// Creates a checked bitmap font.
    ///
    /// # Panics
    /// Panics if its dimensions, baseline, or glyph storage are invalid.
    #[must_use]
    pub const fn new(glyphs: &'glyphs [T], first_glyph: char, width: u8, height: u8, baseline: u8,
        advance_x: u8, advance_y: u8) -> Self {
        let glyph_bits = width as usize * height as usize;
        assert!(width != 0 && height != 0, "bitmap glyph dimensions must be non-zero");
        assert!(baseline < height, "bitmap font baseline must be inside the glyph");
        assert!(glyph_bits <= 64, "bitmap glyph exceeds 64 bits");
        assert!(size_of::<T>() >= glyph_bits.div_ceil(8), "bitmap glyph storage is too small");
        Self {
            glyphs, first_glyph, extra_glyphs: &[], width, height, baseline, advance_x, advance_y
        }
    }
    #[must_use] /// Adds individually mapped glyphs.
    pub const fn with_extra_glyphs(mut self, extra_glyphs: &'glyphs [(char, T)]) -> Self {
        self.extra_glyphs = extra_glyphs;
        self
    }

    /* data */

    #[must_use] /// Returns the sequential glyphs.
    pub const fn glyphs(&self) -> &[T] { self.glyphs }
    #[must_use] /// Returns the first sequential glyph character.
    pub const fn first_glyph(&self) -> char { self.first_glyph }
    #[must_use] /// Returns the individually mapped glyphs.
    pub const fn extra_glyphs(&self) -> &[(char, T)] { self.extra_glyphs }

    /* metrics */

    #[must_use] /// Returns the glyph width.
    pub const fn width(&self) -> u8 { self.width }
    #[must_use] /// Returns the glyph height.
    pub const fn height(&self) -> u8 { self.height }

    #[must_use] /// Returns the zero-based baseline row.
    pub const fn baseline(&self) -> u8 { self.baseline }

    #[must_use] /// Returns the horizontal glyph advance.
    pub const fn advance_x(&self) -> u8 { self.advance_x }
    #[must_use] /// Returns the vertical line advance.
    pub const fn advance_y(&self) -> u8 { self.advance_y }

    #[must_use] /// Returns the bits used by each glyph.
    pub const fn glyph_bits(&self) -> usize { self.width as usize * self.height as usize }

    /* lookup */

    /// Returns a reference to the glyph for `c`.
    ///
    /// The sequential range takes precedence over extra mappings.
    #[must_use]
    pub const fn glyph_ref(&self, c: char) -> Option<&T> {
        let code = c as u32;
        let first = self.first_glyph as u32;
        if code >= first {
            let index = (code - first) as usize;
            if index < self.glyphs.len() { return Some(&self.glyphs[index]); }
        }
        whilst! { i in 0..self.extra_glyphs.len(); {
            if self.extra_glyphs[i].0 as u32 == code { return Some(&self.extra_glyphs[i].1); }
        }}
        None
    }
    #[must_use] /// Returns whether a glyph exists for `c`.
    pub const fn has_glyph(&self, c: char) -> bool {
        self.glyph_ref(c).is_some()
    }

    /* measurement */

    #[must_use] /// Returns the horizontal advance after single-line `text`.
    pub const fn text_advance(&self, text: &str) -> usize {
        CharIter::<&str>::new(text).count().saturating_mul(self.advance_x as usize)
    }
    #[must_use] /// Returns the fixed-glyph span of single-line `text`.
    pub const fn text_width(&self, text: &str) -> usize {
        let count = CharIter::<&str>::new(text).count();
        if count == 0 { 0 }
        else {
            (count - 1).saturating_mul(self.advance_x as usize).saturating_add(self.width as usize)
        }
    }
}

#[rustfmt::skip]
impl<T: Copy + Into<u64>> FontBitmapWord<'_, T> {
    /// Draws text into a one-byte-per-pixel monochrome buffer.
    pub fn draw_mono(&self, buffer: &mut [u8], width: usize, origin: Position2<i64>, text: &str) {
        is! { width == 0, return; }
        let height = buffer.len() / width;
        for pixel in self.text_pixels(origin, text) {
            let [x, y] = pixel.position().dim;
            if x >= 0 && y >= 0 {
                let (x, y) = (x as usize, y as usize);
                if x < width && y < height {
                    buffer[y * width + x] = 1;
                }
            }
        }
    }
    /// Draws text into a four-byte-per-pixel RGBA buffer.
    pub fn draw_rgba(&self, buffer: &mut [u8], width: usize, origin: Position2<i64>,
        text: &str, color: [u8; 4]) {
        let stride = unwrap![some_or width.checked_mul(4), return];
        is! { stride == 0, return }
        let height = buffer.len() / stride;
        for pixel in self.text_pixels(origin, text) {
            let [x, y] = pixel.position().dim;
            if x >= 0 && y >= 0 {
                let (x, y) = (x as usize, y as usize);
                if x < width && y < height {
                    let offset = y * stride + x * 4;
                    buffer[offset..offset + 4].copy_from_slice(&color);
                }
            }
        }
    }
    /// Draws RGBA text using a per-pixel color function.
    pub fn draw_rgba_with<F>(&self, buffer: &mut [u8], width: usize, origin: Position2<i64>,
        text: &str, mut color_fn: F)
    where
        F: FnMut(FontBitmapPixel) -> [u8; 4],
    {
        let stride = unwrap![some_or width.checked_mul(4), return];
        is! { stride == 0, return }
        let height = buffer.len() / stride;
        for pixel in self.text_pixels(origin, text) {
            let [x, y] = pixel.position().dim;
            if x >= 0 && y >= 0 {
                let (x, y) = (x as usize, y as usize);
                if x < width && y < height {
                    let offset = y * stride + x * 4;
                    let color = color_fn(pixel);
                    buffer[offset..offset + 4].copy_from_slice(&color);
                }
            }
        }
    }

    /// Draws text onto a raster canvas.
    #[cfg(all(feature = "draw", feature = "image"))]
    pub fn draw_canvas<C>(&self, canvas: &mut C, origin: Position2<i64>,
        text: &str, color: C::Color) -> Result<(), C::Error>
    where
        C: CanvasRaster,
        C::Color: Copy,
    {
        let grid = canvas.canvas_raster_grid();
        for pixel in self.text_pixels(origin, text) {
            if let Some(coord) = grid.checked_coord(pixel.position()) {
                canvas.canvas_set_color(coord, color)?;
            }
        }
        Ok(())
    }
}
impl<T: Copy> FontBitmapWord<'_, T> {
    /// Returns the glyph for `c`.
    #[must_use]
    pub const fn glyph(&self, c: char) -> Option<T> {
        unwrap![=some_map self.glyph_ref(c), |glyph| *glyph]
    }
    /// Returns the glyph for `c`, or `fallback`.
    #[must_use]
    pub const fn glyph_or(&self, c: char, fallback: T) -> T {
        unwrap![some_or self.glyph(c), fallback]
    }
}

impl<'glyphs, T: Copy> FontBitmapWord<'glyphs, T> {
    /// Returns an iterator over the set pixels of single-line `text`.
    ///
    /// `origin.x` is the left edge of the first glyph
    /// and `origin.y` is the baseline position.
    ///
    /// Characters without a mapped glyph emit no pixels but still consume
    /// the normal horizontal advance. Line breaks, tabs, shaping,
    /// and other text-layout behavior are not interpreted.
    ///
    /// Position arithmetic and character indices saturate
    /// at their respective integer bounds.
    pub const fn text_pixels<'a>(
        &'a self,
        origin: Position2<i64>,
        text: &'a str,
    ) -> FontBitmapPixelIter<'a, 'glyphs, T> {
        FontBitmapPixelIter::new(self, origin, text)
    }
}
