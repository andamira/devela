// devela/src/media/font/bitmap/word.rs
//
//! Defines [`FontBitmapWord`].
//

use crate::{Debug, FmtResult, Formatter};
use crate::{format_buf, whilst};

#[doc = crate::_tags!(font)]
/// A fixed-size bitmap font packed into glyph words.
#[doc = crate::_doc_meta!{
    location("media/font"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: FontBitmapWord<()> = 28|224),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: FontBitmapWord<()> = 48|384),
}]
///
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
        assert!(size_of::<T>() >= (glyph_bits + 7) / 8, "bitmap glyph storage is too small");
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
        match self.glyph_ref(c) { Some(_) => true, None => false }
    }

    /* measurement */

    #[must_use] /// Returns the horizontal advance after single-line `text`.
    // IMPROVE: make const with IterChar
    pub fn text_advance(&self, text: &str) -> usize {
        text.chars().count().saturating_mul(self.advance_x as usize)
    }
    #[must_use] /// Returns the fixed-glyph span of single-line `text`.
    // IMPROVE: make const
    pub fn text_width(&self, text: &str) -> usize {
        let count = text.chars().count();
        if count == 0 { 0 }
        else {
            (count - 1).saturating_mul(self.advance_x as usize).saturating_add(self.width as usize)
        }
    }
}

#[rustfmt::skip]
impl<T: Copy + Into<u64>> FontBitmapWord<'_, T> {
    /// Draws text into a one-byte-per-pixel buffer.
    pub fn draw_mono(&self, buffer: &mut [u8], width: usize, x: isize, y: isize, text: &str) {
        if width == 0 { return; }
        let height = buffer.len() / width;
        self.for_each_pixel_with_local(x, y, text, |pixel_x, pixel_y, _, _, _| {
            if pixel_x >= 0 && pixel_y >= 0 {
                let pixel_x = pixel_x as usize;
                let pixel_y = pixel_y as usize;
                if pixel_x < width && pixel_y < height {
                    buffer[pixel_y * width + pixel_x] = 1;
                }
            }
        });
    }
    /// Draws RGBA text into a four-byte-per-pixel buffer.
    pub fn draw_rgba(&self, buffer: &mut [u8], width: usize, x: isize, y: isize,
        text: &str, color: [u8; 4]) {
        let Some(stride) = width.checked_mul(4) else { return; };
        if stride == 0 { return; }
        let height = buffer.len() / stride;
        self.for_each_pixel_with_local(x, y, text, |pixel_x, pixel_y, _, _, _| {
            if pixel_x >= 0 && pixel_y >= 0 {
                let pixel_x = pixel_x as usize;
                let pixel_y = pixel_y as usize;
                if pixel_x < width && pixel_y < height {
                    let offset = pixel_y * stride + pixel_x * 4;
                    buffer[offset..offset + 4].copy_from_slice(&color);
                }
            }
        });
    }
    /// Draws RGBA text using a per-pixel color function.
    pub fn draw_rgba_with<F>(&self, buffer: &mut [u8], width: usize, x: isize, y: isize,
        text: &str, mut color_fn: F) where F: FnMut(usize, usize, usize) -> [u8; 4] {
        let Some(stride) = width.checked_mul(4) else { return; };
        if stride == 0 { return; }
        let height = buffer.len() / stride;
        self.for_each_pixel_with_local(x, y, text,
            |pixel_x, pixel_y, local_x, local_y, char_index| {
                if pixel_x >= 0 && pixel_y >= 0 {
                    let pixel_x = pixel_x as usize;
                    let pixel_y = pixel_y as usize;
                    if pixel_x < width && pixel_y < height {
                        let offset = pixel_y * stride + pixel_x * 4;
                        let color = color_fn(local_x, local_y, char_index);
                        buffer[offset..offset + 4].copy_from_slice(&color);
                    }
                }
            },
        );
    }

    fn for_each_pixel_with_local<F>(&self, x: isize, y: isize, text: &str, mut f: F)
    where F: FnMut(isize, isize, usize, usize, usize) {
        let (mut x_pos, mut char_index) = (x, 0);
        for c in text.chars() {
            if let Some(glyph) = self.glyph(c) {
                let glyph: u64 = glyph.into();
                for row in 0..self.height {
                    let global_y = y.saturating_add(row as isize)
                        .saturating_sub(self.baseline as isize);
                    for col in 0..self.width {
                        let bit = row * self.width + col;
                        if glyph & (1 << bit) != 0 {
                            let global_x = x_pos.saturating_add(col as isize);
                            f(global_x, global_y, col as usize, row as usize, char_index);
                        }
                    }
                }
            }
            x_pos = x_pos.saturating_add(self.advance_x as isize);
            char_index += 1;
        }
    }
}
#[rustfmt::skip]
impl<T: Copy> FontBitmapWord<'_, T> {
    /// Returns the glyph for `c`.
    #[must_use]
    pub const fn glyph(&self, c: char) -> Option<T> {
        match self.glyph_ref(c) { Some(glyph) => Some(*glyph), None => None }
    }
    /// Returns the glyph for `c`, or `fallback`.
    #[must_use]
    pub const fn glyph_or(&self, c: char, fallback: T) -> T {
        match self.glyph(c) { Some(glyph) => glyph, None => fallback }
    }
}
