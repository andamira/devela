// devela_base_core::text::char::namespace::char

use crate::{ASCII_TABLE, Char};

/// # Methods over `char`
#[rustfmt::skip]
impl Char<char> {
    /// Returns the number of bytes needed to encode the given `char` as UTF-8.
    ///
    /// See also `Char::`[`code_len_utf8`][Char::code_len_utf8].
    #[must_use] #[rustfmt::skip]
    pub const fn len_utf8(self) -> usize {
        let code = self.0 as u32;
        if code < 0x80 { 1 } else if code < 0x800 { 2 } else if code < 0x10_000 { 3 } else { 4 }
    }

    /// Converts the given `char` to a UTF-8 encoded byte sequence.
    ///
    /// Always returns a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`char::encode_utf8`].
    #[must_use]
    pub const fn to_utf8_bytes(self) -> [u8; 4] {
        Char(self.0 as u32).code_to_utf8_bytes_unchecked()
    }

    /// Returns the ASCII representation as a `&'static str`, or `""` if non-ASCII.
    #[must_use]
    pub const fn to_ascii_str(self) -> &'static str {
        if self.0.is_ascii() { ASCII_TABLE[self.0 as usize] } else { "" }
    }
    /// Returns the ASCII representation as a `&'static str`.
    ///
    /// # Panics
    /// Panics if the character is not ASCII.
    #[must_use]
    pub const fn to_ascii_str_unchecked(self) -> &'static str { ASCII_TABLE[self.0 as usize] }

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
            'Æ' => Some('A'), 'æ' => Some('a'),
            'Œ' => Some('O'), 'œ' => Some('o'),
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
    #[must_use]
    pub const fn to_ascii_fold_unchecked(self) -> char {
        if let Some(m) = self.to_ascii_fold() { m } else { self.0 }
    }
}
