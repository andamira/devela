// devela::media::bitmap::bitmap
//
//! Defines the [`FontBitmap`] struct.
//
// TODO
// - wrapping.
// - max width.

use crate::{format_buf, is};

#[doc = crate::_tags!(font)]
/// A simple bitmap font for rendering fixed-size glyphs.
#[doc = crate::_doc_location!("media/font")]
///
/// Each glyph is stored as a bitfield in a generic type and is assumed to have
/// fixed dimensions (`width` × `height`), a baseline, and an advance metric.
///
/// The glyphs are arranged sequentially starting from `first_glyph`.
///
/// The font supports drawing text into both mono and RGBA buffers,
/// as well as using a custom per-pixel color function.
#[derive(Clone, PartialEq, Eq, Hash)] //, Debug,
pub struct FontBitmap<'glyphs, T> {
    /// A slice of glyphs.
    pub glyphs: &'glyphs [T],
    /// The first char in `glyphs`.
    pub first_glyph: char,

    /// A slice of extra paired glyphs.
    pub extra_glyphs: &'glyphs [(char, T)],

    /// The width of each glyph in pixels.
    pub width: u8,
    /// The height of each glyph in pixels.
    pub height: u8,
    /// Where the base line sits in the height.
    pub baseline: u8,
    /// Horizontal space to advance after each glyph.
    pub advance_x: u8,
    /// Vertical space to advance after each new line.
    pub advance_y: u8,
}

impl<T> core::fmt::Debug for FontBitmap<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut buf = [0u8; 128];
        let name = format_buf![&mut buf, "FontBitmap<{}>", stringify!(T)].unwrap();
        f.debug_struct(name)
            .field("glyphs", &self.glyphs.len())
            .field("first_glyph", &self.first_glyph)
            .field("extra_glyphs", &self.extra_glyphs.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("baseline", &self.baseline)
            .field("advance_x", &self.advance_x)
            .field("advance_y", &self.advance_x)
            .finish()
    }
}

impl<T: Copy + Into<u64>> FontBitmap<'_, T> {
    /// Returns the rendered text width.
    pub fn text_width(&self, text: &str) -> usize {
        text.chars().count() * self.advance_x as usize
    }

    /// Returns the height of any glyph.
    pub const fn height(&self) -> u8 {
        self.height
    }
    /// Returns the width of any glyph.
    pub const fn width(&self) -> u8 {
        self.width
    }

    /// Draws grayscale text into a one-byte-per-pixel buffer.
    pub fn draw_mono(&self, buffer: &mut [u8], width: usize, x: isize, y: isize, text: &str) {
        let height = buffer.len() / width;
        self.for_each_pixel_with_local(x, y, text, |pixel_x, pixel_y, _, _, _| {
            if pixel_x >= 0 && pixel_x < width as isize && pixel_y >= 0 && pixel_y < height as isize
            {
                let offset = (pixel_y as usize) * width + (pixel_x as usize);
                buffer[offset] = 1;
            }
        });
    }

    /// Draws RGBA text into a 4-bytes-per-pixel buffer.
    pub fn draw_rgba(
        &self,
        buffer: &mut [u8],
        width: usize,
        x: isize,
        y: isize,
        text: &str,
        color: [u8; 4],
    ) {
        let height = buffer.len() / (width * 4);
        self.for_each_pixel_with_local(x, y, text, |pixel_x, pixel_y, _, _, _| {
            if pixel_x >= 0 && pixel_x < width as isize && pixel_y >= 0 && pixel_y < height as isize
            {
                let offset = ((pixel_y as usize) * width + (pixel_x as usize)) * 4;
                buffer[offset..offset + 4].copy_from_slice(&color);
            }
        });
    }

    /// Draws RGBA text with a custom color function.
    ///
    /// The provided closure is called for each "on" pixel and receives the glyph‑local
    /// x and y coordinates (i.e. within the glyph) and the index of the current character.
    /// It should return a `[u8; 4]` color (RGBA) for that pixel.
    pub fn draw_rgba_with<F>(
        &self,
        buffer: &mut [u8],
        width: usize,
        x: isize,
        y: isize,
        text: &str,
        mut color_fn: F,
    ) where
        F: FnMut(usize, usize, usize) -> [u8; 4],
    {
        let height = buffer.len() / (width * 4);
        self.for_each_pixel_with_local(
            x,
            y,
            text,
            |global_x, global_y, local_x, local_y, char_index| {
                if global_x >= 0
                    && global_x < width as isize
                    && global_y >= 0
                    && global_y < height as isize
                {
                    let color = color_fn(local_x, local_y, char_index);
                    let offset = ((global_y as usize) * width + (global_x as usize)) * 4;
                    buffer[offset..offset + 4].copy_from_slice(&color);
                }
            },
        );
    }
}

// private methods
impl<T: Copy + Into<u64>> FontBitmap<'_, T> {
    /// Iterates over every pixel that should be drawn for the given text.
    ///
    /// The closure receives:
    /// - `global_x` and `global_y`: the final buffer coordinates.
    /// - `local_x` and `local_y`: the coordinates within the current glyph.
    /// - `char_index`: the index of the current character in the text.
    fn for_each_pixel_with_local<F>(&self, x: isize, y: isize, text: &str, mut f: F)
    where
        F: FnMut(isize, isize, usize, usize, usize),
    {
        let mut x_pos = x;
        let mut char_index = 0;
        for c in text.chars() {
            if let Some(glyph_index) = (c as usize).checked_sub(self.first_glyph as usize) {
                if glyph_index < self.glyphs.len() {
                    let glyph: u64 = self.glyphs[glyph_index].into();
                    for row in 0..self.height {
                        let global_y = y + row as isize - self.baseline as isize;
                        is![global_y < 0, continue];
                        for col in 0..self.width {
                            let global_x = x_pos + col as isize;
                            let bit_pos = row * self.width + col;
                            // this would read rows top to bottom, draw pixels left to right
                            // (self.height - 1 - row) * self.width + (self.width - 1 - col);
                            if (glyph & (1 << bit_pos)) != 0 {
                                f(global_x, global_y, col as usize, row as usize, char_index);
                            }
                        }
                    }
                }
            }
            x_pos += self.advance_x as isize;
            char_index += 1;
        }
    }
}
