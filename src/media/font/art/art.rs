// devela::media::font::art::art
//
//! Defines the [`FontArt`] struct.
//
// MAYBE: use utf8_char or char7 or char8? for first and last glyphs
// MAYBE:RENAME: methods: get_glyph_... etc.

use crate::{Lut, is, unwrap};

#[doc = crate::_TAG_FONT!()]
/// A simple Unicode-Art font for rendering fixed-size glyphs.
///
/// The glyphs are arranged sequentially starting from `first_glyph`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)] //, Debug,
pub struct FontArt<'g> {
    /// A slice of glyphs.
    pub glyphs: &'g [&'g [&'g str]],
    /// The first char in `glyphs`.
    pub first_glyph: char,
    /// The last char in `glyphs`.
    pub last_glyph: char,

    // /// A slice of extra paired glyphs.
    // pub extra_glyphs: &'glyphs [(char, T)],
    /// The width of each glyph in text cells.
    pub width: u8,
    /// The height of each glyph in text cells.
    pub height: u8,
    /// Where the base line sits in the height.
    pub baseline: u8,
    /// Horizontal space to advance after each glyph.
    pub advance_x: u8,
    /// Vertical space to advance after each new line.
    pub advance_y: u8,
}

impl<'g> FontArt<'g> {
    /// Returns `true` if the font has the corresponding glyph.
    pub const fn has_glyph(&self, c: char) -> bool {
        let idx = (c as u32).wrapping_sub(self.first_glyph as u32);
        let max_idx = self.last_glyph as u32 - self.first_glyph as u32;
        idx <= max_idx
    }

    /// Returns the glyph for the specified decimal digit (0-9).
    ///
    /// # Panics
    /// Panics if `digit` is not between 0 and 9, or if the font was constructed incorrectly.
    #[must_use]
    pub const fn get_digit(&self, digit: u8) -> &'g [&'g str] {
        assert!(digit <= 9, "Digit must be between 0 and 9");
        let char_code = self.first_glyph as u8 + digit;
        self.get_glyph_from_scalar_unchecked(char_code as u32)
    }

    ///
    #[must_use]
    pub const fn get_digit_base(&self, digit: u8, base: u8) -> &'g [&'g str] {
        assert!(base >= 2 && base <= 36, "Base must be between 2 and 36");
        assert!(digit < base, "The given digit is not valid in the given base");
        let offset = Lut::ASCII_BASE36_OFFSET[digit as usize];
        let char_code = self.first_glyph as u8 + digit + offset;
        self.get_glyph_from_scalar_unchecked(char_code as u32)
    }

    /// Returns the glyph for the specified character, if available.
    ///
    /// Returns `Some(glyph)` if `c` is within [`first_glyph`..=`last_glyph`],
    ///
    /// or `None` if the character is not supported by this font.
    ///
    /// # Panics
    /// Panics only if the font was constructed incorrectly and `glyphs` does not
    /// contain exactly the characters from `first_glyph` to `last_glyph` inclusive.
    #[must_use]
    pub const fn get_glyph(&self, c: char) -> Option<&'g [&'g str]> {
        let idx = (c as usize).wrapping_sub(self.first_glyph as usize);
        let max_idx = self.last_glyph as usize - self.first_glyph as usize;
        is![idx <= max_idx; Some(self.glyphs[idx]); None]
    }

    /// Returns the glyph for the specified Unicode scalar, if available.
    ///
    /// Returns `Some(glyph)` if `c` is within [`first_glyph`..=`last_glyph`],
    /// or `None` if the character is not supported by this font.
    ///
    /// # Panics
    /// Panics only if the font was constructed incorrectly and `glyphs` does not
    /// contain exactly the characters from `first_glyph` to `last_glyph` inclusive.
    pub const fn get_glyph_from_scalar(&self, c: u32) -> Option<&'g [&'g str]> {
        let idx = c.wrapping_sub(self.first_glyph as u32) as usize;
        let max_idx = self.last_glyph as usize - self.first_glyph as usize;
        is![idx <= max_idx; Some(self.glyphs[idx]); None]
    }
    /// Returns the glyph for the specified Unicode scalar, if available.
    ///
    /// Returns `Some(glyph)` if `c` is within [`first_glyph`..=`last_glyph`],
    /// or `None` if the character is not supported by this font.
    ///
    /// # Panics
    /// Panics only if the font was constructed incorrectly and `glyphs` does not
    /// contain exactly the characters from `first_glyph` to `last_glyph` inclusive.
    pub const fn get_glyph_from_scalar_unchecked(&self, c: u32) -> &'g [&'g str] {
        let idx = c.wrapping_sub(self.first_glyph as u32) as usize;
        &self.glyphs[idx]
        // let max_idx = self.last_glyph as usize - self.first_glyph as usize;
        // is![idx <= max_idx; Some(self.glyphs[idx]); None]
    }

    // MAYBE RENAME? from_char? from_scalar?
    // TODO: unchecked

    // MAYBE NOT:
    /// Returns the glyph for the specified character.
    ///
    /// # Panics
    /// Panics if:
    /// - The character is not within [`first_glyph`..=`last_glyph`], or
    /// - The font was constructed incorrectly and `glyphs` does not contain
    ///   exactly the characters from `first_glyph` to `last_glyph` inclusive.
    pub const fn get_glyph_unchecked(&self, c: char) -> &'g [&'g str] {
        let idx = (c as usize) - (self.first_glyph as usize);
        &self.glyphs[idx]
    }

    /// Returns the glyph for a character, or a fallback/default glyph
    pub const fn get_glyph_or(&self, c: char, default: &'g [&'g str]) -> &'g [&'g str] {
        unwrap![some_or self.get_glyph(c), default]
    }

    /// Returns the glyph for a character, or the first glyph as fallback
    pub const fn get_glyph_or_first(&self, c: char) -> &'g [&'g str] {
        unwrap![some_or self.get_glyph(c), self.glyphs[0]]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get() {}
}
