// devela/src/media/font/bitmap/view.rs
//
//! Defines [`FontBitmapView`] and [`GlyphBitmapView`].

use crate::{CharIter, Debug, FmtResult, Fonts, Formatter, Region2, Slice};

#[doc = crate::_tags!(font)]
/// A validated, borrowed view over fixed-metric monochrome bitmap-font data.
#[doc = crate::_doc_meta!{
    location("media/font"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: FontBitmapView = 44|352),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: FontBitmapView = 64|512),
}]
/// Every glyph shares the same dimensions and metrics. Unicode scalar values
/// are stored in sorted order and map one-to-one to glyph bitmap records.
///
/// Bitmap rows run from top to bottom. Within each row, pixels run from left
/// to right, beginning at the most-significant bit of each byte.
///
/// The view borrows its backing storage and performs no allocation or copying.
#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontBitmapView<'a> {
    /// Sorted little-endian Unicode scalar values, four bytes per mapped glyph.
    pub(crate) scalars_le: &'a [u8],
    /// Concatenated fixed-metric monochrome glyph bitmap records.
    pub(crate) bitmaps: &'a [u8],
    /// Number of bytes between the starts of consecutive glyph records.
    pub(crate) glyph_stride: u32,
    /// Fixed visible glyph width, in pixels.
    pub(crate) width: u16,
    /// Fixed visible glyph height, in pixels.
    pub(crate) height: u16,
    /// Fixed visible glyph height, in pixels.
    pub(crate) row_stride: u16,
    /// Horizontal offset from the glyph origin to the bitmap's left edge.
    pub(crate) bounds_x: i16,
    /// Vertical offset from the baseline to the bitmap's bottom edge.
    pub(crate) bounds_y: i16,
    /// Horizontal distance between consecutive glyph origins.
    pub(crate) advance_x: u16,
    /// Vertical distance between consecutive line origins.
    pub(crate) line_advance: u16,
    /// Font-wide ascent above the baseline.
    pub(crate) ascent: u16,
    /// Font-wide descent below the baseline.
    pub(crate) descent: u16,
    /// Optional mapped character used when a requested glyph is absent.
    pub(crate) default_character: Option<char>,
}
impl Debug for FontBitmapView<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        f.debug_struct("FontBitmapView")
            .field("glyph_count", &self.glyph_count())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("row_stride", &self.row_stride)
            .field("glyph_stride", &self.glyph_stride)
            .field("bounds_x", &self.bounds_x)
            .field("bounds_y", &self.bounds_y)
            .field("advance_x", &self.advance_x)
            .field("line_advance", &self.line_advance)
            .field("ascent", &self.ascent)
            .field("descent", &self.descent)
            .field("default_character", &self.default_character)
            .finish()
    }
}
#[rustfmt::skip]
impl<'a> FontBitmapView<'a> {
    #[must_use] /// Returns the number of mapped glyphs.
    pub const fn glyph_count(&self) -> usize { self.scalars_le.len() / 4 }
    #[must_use] /// Returns the fixed visible glyph width, in pixels.
    pub const fn width(&self) -> u16 { self.width }
    #[must_use] /// Returns the fixed visible glyph height, in pixels.
    pub const fn height(&self) -> u16 { self.height }
    #[must_use]
    /// Returns the number of bytes separating consecutive bitmap rows.
    ///
    /// This may be larger than the minimum number of bytes needed for `width`
    /// pixels when the backing storage pads its rows.
    pub const fn row_stride(&self) -> usize { self.row_stride as usize }
    #[must_use]
    /// Returns the number of bytes separating consecutive glyph records.
    ///
    /// This may be larger than [`glyph_bitmap_len`][Self::glyph_bitmap_len]
    /// when the backing storage places padding between glyphs.
    pub const fn glyph_stride(&self) -> usize { self.glyph_stride as usize }

    #[must_use] /// Returns the number of bitmap bytes exposed for each glyph.
    pub const fn glyph_bitmap_len(&self) -> usize { self.row_stride() * self.height as usize }

    /// Returns the fixed glyph bounding region in font coordinates.
    pub const fn bounds(&self) -> Region2<i16, u16> {
        Region2::from_xy_wh(self.bounds_x, self.bounds_y, self.width, self.height)
    }
    #[must_use] /// Returns the horizontal distance between consecutive glyph origins.
    pub const fn advance_x(&self) -> u16 { self.advance_x }
    #[must_use] /// Returns the vertical distance between consecutive line origins.
    pub const fn line_advance(&self) -> u16 { self.line_advance }
    #[must_use] /// Returns the font ascent above the baseline.
    pub const fn ascent(&self) -> u16 { self.ascent }
    #[must_use] /// Returns the font descent below the baseline.
    pub const fn descent(&self) -> u16 { self.descent }

    #[must_use]
    /// Returns the signed offset from the bitmap's top edge to the baseline.
    ///
    /// This is derived from the bitmap bounding region,
    /// independently of the font-wide ascent and descent metrics.
    pub const fn baseline_from_top(&self) -> i32 {
        self.height as i32 + self.bounds_y as i32
    }

    #[must_use]
    /// Returns the fallback character, when the font defines one.
    pub const fn default_character(&self) -> Option<char> { self.default_character }
    #[must_use]
    /// Returns the Unicode scalar value of the fallback character, when present.
    pub const fn default_scalar(&self) -> Option<u32> {
        match self.default_character {
            Some(c) => Some(c as u32),
            None => None,
        }
    }
    #[must_use]
    /// Returns the fallback glyph, when the font defines one.
    pub const fn default_glyph(&self) -> Option<GlyphBitmapView<'a>> {
        match self.default_character {
            Some(c) => self.glyph(c),
            None => None,
        }
    }

    #[must_use] /// Returns whether the font contains a glyph for `character`.
    pub const fn has_glyph(&self, c: char) -> bool { self.glyph_index_scalar(c as u32).is_some() }
    #[must_use] 
    /// Returns whether the font contains a glyph for `scalar`.
    ///
    /// Invalid Unicode scalar values simply return `false`.
    pub const fn has_glyph_scalar(&self, scalar: u32) -> bool {
        self.glyph_index_scalar(scalar).is_some()
    }

    /// Returns the glyph mapped to `character`.
    pub const fn glyph(&self, character: char) -> Option<GlyphBitmapView<'a>> {
        self.glyph_scalar(character as u32)
    }

    /// Returns the glyph mapped to the Unicode `scalar` value.
    ///
    /// Invalid or unmapped scalar values return `None`.
    pub const fn glyph_scalar(&self, scalar: u32) -> Option<GlyphBitmapView<'a>> {
        match self.glyph_index_scalar(scalar) {
            Some(index) => self.glyph_at(index),
            None => None,
        }
    }
    /// Returns the mapped glyph for `character`, or the fallback glyph.
    ///
    /// Returns `None` when neither glyph exists.
    pub const fn glyph_or_default(&self, character: char) -> Option<GlyphBitmapView<'a>> {
        match self.glyph(character) {
            Some(glyph) => Some(glyph),
            None => self.default_glyph(),
        }
    }

    /// Returns the glyph at `index` in scalar-sort order.
    pub const fn glyph_at(&self, index: usize) -> Option<GlyphBitmapView<'a>> {
        let character = match self.character_at(index) {
            Some(character) => character,
            None => return None,
        };
        let start = match index.checked_mul(self.glyph_stride()) {
            Some(start) => start,
            None => return None,
        };
        let end = match start.checked_add(self.glyph_bitmap_len()) {
            Some(end) => end,
            None => return None,
        };
        if end > self.bitmaps.len() { return None; }
        Some(GlyphBitmapView {
            character,
            bitmap: Slice::range(self.bitmaps, start, end),
            width: self.width,
            height: self.height,
            row_stride: self.row_stride,
        })
    }
    /// Returns the Unicode scalar value at `index` in scalar-sort order.
    pub const fn scalar_at(&self, index: usize) -> Option<u32> {
        if index < self.glyph_count() { Some(self.scalar_at_unchecked(index)) }
        else { None }
    }
    /// Returns the character at `index` in scalar-sort order.
    pub const fn character_at(&self, index: usize) -> Option<char> {
        match self.scalar_at(index) {
            Some(scalar) => char::from_u32(scalar),
            None => None,
        }
    }

    /// Returns the scalar-sort index of `character`.
    pub const fn glyph_index(&self, character: char) -> Option<usize> {
        self.glyph_index_scalar(character as u32)
    }
    /// Returns the scalar-sort index of the Unicode `scalar` value.
    ///
    /// The lookup performs a binary search over the sorted scalar map.
    pub const fn glyph_index_scalar(&self, scalar: u32) -> Option<usize> {
        let glyph_count = self.glyph_count();
        let (mut low, mut high) = (0usize, glyph_count);
        while low < high {
            let mid = low + (high - low) / 2;
            let current = self.scalar_at_unchecked(mid);
            if current < scalar { low = mid + 1; } else { high = mid; }
        }
        if low < glyph_count && self.scalar_at_unchecked(low) == scalar { Some(low) } else { None }
    }

    /// Returns the horizontal advance of a single-line text sequence.
    ///
    /// Every Unicode scalar is counted literally; this method does not interpret
    /// line breaks, tabs, combining behavior, or fallback availability.
    pub fn text_advance(&self, text: &str) -> usize {
        CharIter::<&str>::new(text).count().saturating_mul(self.advance_x as usize)
    }

    /* private helpers */

    /// Reads a scalar after the caller has checked the glyph index.
    const fn scalar_at_unchecked(&self, index: usize) -> u32 {
        Fonts::read_u32(self.scalars_le, index * 4)
    }
}

#[doc = crate::_tags!(font)]
/// A borrowed view over one fixed-size monochrome glyph bitmap.
#[doc = crate::_doc_meta!{
    location("media/font"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: GlyphBitmapView = 20|160),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: GlyphBitmapView = 32|256),
}]
/// Rows run from top to bottom. Pixels within each row run from left to right,
/// beginning at the most-significant bit of each byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlyphBitmapView<'a> {
    character: char,
    bitmap: &'a [u8],
    width: u16,
    height: u16,
    row_stride: u16,
}
#[rustfmt::skip]
impl<'a> GlyphBitmapView<'a> {
    /// Returns this glyph's character.
    pub const fn character(&self) -> char { self.character }
    /// Returns this glyph's Unicode scalar value.
    pub const fn scalar(&self) -> u32 { self.character as u32 }

    /// Returns the complete row-addressable bitmap storage for this glyph.
    ///
    /// Its length is `row_stride × height` and excludes any padding between
    /// consecutive glyph records.
    pub const fn bitmap(&self) -> &'a [u8] { self.bitmap }

    /// Returns the visible bitmap width, in pixels.
    pub const fn width(&self) -> u16 { self.width }

    /// Returns the visible bitmap height, in pixels.
    pub const fn height(&self) -> u16 { self.height }

    /// Returns the number of bytes separating consecutive bitmap rows.
    pub const fn row_stride(&self) -> usize { self.row_stride as usize }

    /// Returns the bitmap bytes for `row`, indexed from the top.
    pub const fn row(&self, row: usize) -> Option<&'a [u8]> {
        if row >= self.height as usize { return None; }
        let start = row * self.row_stride();
        Some(Slice::range(self.bitmap, start, start + self.row_stride()))
    }
    /// Returns whether the pixel at `(x, y)` is set.
    ///
    /// Coordinates begin at the bitmap's top-left corner. Returns `None` when
    /// either coordinate lies outside the visible bitmap dimensions.
    pub const fn is_set(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width as usize || y >= self.height as usize { return None; }
        let byte = self.bitmap[y * self.row_stride() + x / 8];
        Some(byte & (0x80 >> (x % 8)) != 0)
    }
}
