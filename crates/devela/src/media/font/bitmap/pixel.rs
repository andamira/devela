//
//! Defines [`FontBitmapWord`].
//

use crate::{CharIter, FontBitmapWord, IteratorFused, Position2, is, unwrap};

#[doc = crate::_tags!(font iterator)]
/// Iterator over the set pixels of text rendered with a [`FontBitmapWord`].
#[doc = crate::_doc_meta!{
    location("media/font", struct FontBitmapPixelIter),
    #[cfg(target_pointer_width = "32")]
    test_size_of(FontBitmapPixelIter<u8> = 56|448; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(FontBitmapPixelIter<u8> = 72|576; niche Option),
}]
/// Pixels are emitted in glyph order and, within each glyph, in row-major
/// order from top-left to bottom-right.
///
/// Characters without a mapped glyph emit no pixels but still consume the
/// font's horizontal advance.
///
/// This iterator performs positioning only. It is independent of destination
/// storage, color, clipping, compositing, and presentation.
#[must_use]
#[derive(Clone, Debug)]
pub struct FontBitmapPixelIter<'a, 'glyphs, T> {
    font: &'a FontBitmapWord<'glyphs, T>,
    chars: CharIter<'a, &'a str>,
    origin_y: i64,
    next_x: i64,
    glyph_x: i64,
    glyph: u64,
    character: char,
    char_index: u32,
}
#[rustfmt::skip]
impl<'a, 'glyphs, T> FontBitmapPixelIter<'a, 'glyphs, T> {
    pub(crate) const fn new(
        font: &'a FontBitmapWord<'glyphs, T>,
        origin: Position2<i64>,
        text: &'a str,
    ) -> Self {
        Self {
            font,
            chars: CharIter::<&str>::new(text),
            origin_y: origin.dim[1],
            next_x: origin.dim[0],
            glyph_x: origin.dim[0],
            glyph: 0,
            character: '\0',
            char_index: 0,
        }
    }

    /// Returns the next set pixel already loaded from the current glyph.
    const fn next_glyph_pixel(&mut self) -> Option<FontBitmapPixel> {
        is! { self.glyph == 0, return None }
        let bit = self.glyph.trailing_zeros() as u8;
        self.glyph &= self.glyph - 1;
        let (row, col) = (bit / self.font.width(), bit % self.font.width());
        let pixel = FontBitmapPixel::new(
            Position2::new([
                self.glyph_x.saturating_add(col as i64),
                self.origin_y
                    .saturating_add(row as i64)
                    .saturating_sub(self.font.baseline() as i64),
            ]),
            Position2::new([col as u16, row as u16]),
            self.character,
            self.char_index,
        );
        is! { self.glyph == 0, self.char_index = self.char_index.saturating_add(1) }
        Some(pixel)
    }
    /// Advances to the next input character and returns its glyph origin.
    const fn next_character(&mut self) -> Option<(char, i64)> {
        let character = unwrap![some? self.chars.next_char()];
        let glyph_x = self.next_x;
        self.next_x = self.next_x.saturating_add(self.font.advance_x() as i64);
        self.char_index = self.char_index.saturating_add(1);
        Some((character, glyph_x))
    }
    const fn load_glyph(&mut self, character: char, glyph_x: i64, glyph: u64) {
        let bits = self.font.glyph_bits();
        self.glyph = is![bits == 64, glyph, glyph & ((1_u64 << bits) - 1)];
        self.glyph_x = glyph_x;
        self.character = character;
    }
}
macro_rules! impl_font_bitmap_word_pixel_iter {
    ($($T:ty),+ $(,)?) => {
        $(
            impl<'a, 'glyphs> FontBitmapPixelIter<'a, 'glyphs, $T> {
                /// Returns the next set bitmap pixel.
                pub const fn next(&mut self) -> Option<FontBitmapPixel> {
                    loop {
                        if let Some(pixel) = self.next_glyph_pixel() { return Some(pixel); }
                        let (character, glyph_x) = unwrap![some? self.next_character()];
                        let glyph = unwrap![some_or self.font.glyph(character), continue];
                        self.load_glyph(character, glyph_x, glyph as u64);
                    }
                }
            }
        )+
    };
}
impl_font_bitmap_word_pixel_iter![u8, u16, u32, u64];

impl<'a, 'glyphs, T> Iterator for FontBitmapPixelIter<'a, 'glyphs, T>
where
    T: Copy + Into<u64>,
{
    type Item = FontBitmapPixel;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pixel) = self.next_glyph_pixel() {
                return Some(pixel);
            }
            let (character, glyph_x) = unwrap![some? self.next_character()];
            let glyph = unwrap![some_or self.font.glyph(character), continue];
            self.load_glyph(character, glyph_x, glyph.into());
        }
    }
}
impl<'a, 'glyphs, T> IteratorFused for FontBitmapPixelIter<'a, 'glyphs, T> where T: Copy + Into<u64> {}

#[doc = crate::_tags!(font)]
/// One set pixel produced while rendering monochrome bitmap text.
#[doc = crate::_doc_meta!{
    location("media/font", struct FontBitmapPixel),
    #[cfg(target_pointer_width = "32")]
    test_size_of(FontBitmapPixel = 28|224; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(FontBitmapPixel = 32|256; niche Option),
}]
/// This describes a font-rendering result independently of any destination
/// raster, pixel storage, color, or compositing model.
///
/// `position` is the signed destination-space position.
/// `glyph_position` is relative to the visible bitmap of the source glyph.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontBitmapPixel {
    position: Position2<i64>,
    glyph_position: Position2<u16>,
    character: char,
    char_index: u32,
}

#[rustfmt::skip]
impl FontBitmapPixel {
    pub(crate) const fn new(
        position: Position2<i64>,
        glyph_position: Position2<u16>,
        character: char,
        char_index: u32,
    ) -> Self {
        Self { position, glyph_position, character, char_index }
    }

    /// Returns the destination-space position.
    pub const fn position(self) -> Position2<i64> { self.position }

    /// Returns the position within the visible glyph bitmap.
    pub const fn glyph_position(self) -> Position2<u16> { self.glyph_position }

    /// Returns the corresponding input character.
    pub const fn character(self) -> char { self.character }

    /// Returns the zero-based Unicode-scalar index in the input text.
    ///
    /// The index saturates at `u32::MAX`.
    pub const fn char_index(self) -> u32 { self.char_index }
}
