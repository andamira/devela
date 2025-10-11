// devela_base_core::text::char::namespace::char
//
// TOC
// - methods over char

use crate::{Char, LUT_ASCII_CHARS, is, unwrap};

/// # Methods over `char`
#[rustfmt::skip]
impl Char<char> {
    /// Returns the number of bytes needed to encode the given Unicode scalar as UTF-8.
    ///
    /// See also `Char::<u32>`[`len_utf8`][Char::<u32>::len_utf8].
    #[must_use]
    pub const fn len_utf8(self) -> usize {
        match self.0 as u32 { 0..0x80 => 1, 0x80..0x800 => 2, 0x800..0x10_000 => 3, _ => 4 }
    }

    /// Returns the monospace display width.
    ///
    /// - 0: Non-printing characters (controls, combining marks)
    /// - 1: Regular characters (Latin, Greek, Cyrillic, etc.)
    /// - 2: Wide characters (CJK, emoji, fullwidth forms)
    #[must_use] #[inline(always)]
    pub const fn width(self) -> usize { Char(self.0 as u32).width() }

    /// Returns the monospace display width using faster calculation.
    ///
    /// Uses optimized checks that cover common cases but may incorrectly
    /// report some obscure Unicode characters as 1 width instead of 2.
    #[must_use] #[inline(always)]
    pub const fn width_common(self) -> usize { Char(self.0 as u32).width_common() }

    /// Returns `true` for all Unicode combining characters.
    ///
    /// Includes musical notation, historic scripts, and obscure diacritics.
    /// Comprehensive but slightly slower than `is_combining_common`.
    #[must_use] #[inline(always)]
    pub const fn is_combining(self) -> bool { Char(self.0 as u32).is_combining() }

    /// Returns `true` for common combining marks used in modern text.
    ///
    /// Covers Latin, Greek, and most European language diacritics.
    /// Fast and suitable for 95% of use cases.
    #[must_use] #[inline(always)]
    pub const fn is_combining_common(self) -> bool { Char(self.0 as u32).is_combining_common() }

    /// Returns `true` for all Unicode control characters.
    #[must_use] #[inline(always)]
    pub const fn is_control(self) -> bool { Char(self.0 as u32).is_control() }

    /// Returns `true` for common Unicode control characters.
    ///
    /// Just ASCII, zero-width spaces, bidi formatting, word joiners and invisible operators.
    #[must_use] #[inline(always)]
    pub const fn is_control_common(self) -> bool { Char(self.0 as u32).is_control_common() }

    /// Returns `true` for all Unicode fullwidth characters.
    #[must_use] #[inline(always)]
    pub const fn is_fullwidth(self) -> bool { Char(self.0 as u32).is_fullwidth() }

    /// Returns `true` for common fullwidth characters (ASCII variants, basic CJK)
    #[must_use] #[inline(always)]
    pub const fn is_fullwidth_common(self) -> bool { Char(self.0 as u32).is_fullwidth_common() }

    /// Converts the given Unicode scalar to a UTF-8 encoded byte sequence.
    ///
    /// Always returns a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`char::encode_utf8`].
    #[must_use] #[inline(always)]
    pub const fn to_utf8_bytes(self) -> [u8; 4] { Char(self.0 as u32).to_utf8_bytes_unchecked() }

    /// Returns the ASCII representation as a `&'static str`, or `""` if non-ASCII.
    #[must_use]
    pub const fn as_ascii(self) -> &'static str {
        is![self.0.is_ascii(); LUT_ASCII_CHARS[self.0 as usize]; ""]
    }

    /// Returns the ASCII representation as a `&'static str`.
    /// # Panics
    /// Panics if the character is not ASCII.
    #[must_use] #[inline(always)]
    pub const fn as_ascii_unchecked(self) -> &'static str { LUT_ASCII_CHARS[self.0 as usize] }

    /// Converts a character to its closest ASCII equivalent, if possible.
    ///
    /// This function attempts to replace accented or special characters with
    /// their ASCII counterparts. If a mapping exists, it returns `Some(char)`,
    /// otherwise, it returns `None`.
    #[must_use]
    pub const fn to_ascii_fold(self) -> Option<char> {
        match self.0 {
            // ASCII already, return as-is
            _ if self.0.is_ascii() => Some(self.0),
            // Latin-1 Supplement
            'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => Some('A'),
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => Some('a'),
            'È' | 'É' | 'Ê' | 'Ë' => Some('E'),
            'è' | 'é' | 'ê' | 'ë' => Some('e'),
            'Ì' | 'Í' | 'Î' | 'Ï' => Some('I'),
            'ì' | 'í' | 'î' | 'ï' => Some('i'),
            'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' => Some('O'),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' => Some('o'),
            'Ù' | 'Ú' | 'Û' | 'Ü' => Some('U'),
            'ù' | 'ú' | 'û' | 'ü' => Some('u'),
            'Ý' | 'Ÿ' => Some('Y'),
            'ý' | 'ÿ' => Some('y'),
            'Ç' => Some('C'),
            'ç' => Some('c'),
            'Ñ' => Some('N'),
            'ñ' => Some('n'),
            // Ligatures & Special Cases
            'Æ' | 'Œ' => Some('E'),
            'æ' | 'œ' => Some('e'),
            // Symbols that could be mapped
            'ß' => Some('s'), // German sharp S → s
            'Ð' => Some('D'), 'ð' => Some('d'),
            'Þ' => Some('P'), 'þ' => Some('p'),
            _ => None, // No reasonable ASCII mapping
        }
    }
    /// Converts a character to its closest ASCII equivalent,
    /// or returns the input character if no mapping exists.
    ///
    /// This function is similar to
    /// [`to_ascii_fold`][Self::to_ascii_fold], but **never returns `None`**.
    /// If no ASCII equivalent exists, the input character is returned unchanged.
    #[must_use] #[inline(always)]
    pub const fn to_ascii_fold_unchecked(self) -> char {
        unwrap![some_or self.to_ascii_fold(), self.0]
    }
}
