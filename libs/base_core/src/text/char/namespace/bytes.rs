// devela_base_core::text::char::namespace::bytes
//
// TOC
// - methods over u8
// - methods over &[u8]
// - methods over &[u8; N]

use crate::{Char, is, unwrap};

/// # Methods over `u8`.
#[rustfmt::skip]
impl Char<u8> {
    /* private helpers */

    /// Bitmask for extracting the 6-bit payload from a UTF-8 continuation byte (`10xxxxxx`).
    pub(crate) const CONT_MASK: u8 = 0b0011_1111;

    // https://tools.ietf.org/html/rfc3629
    // https://github.com/rust-lang/rust/blob/master/library/core/src/str/validations.rs
    const UTF8_CHAR_LEN: &[u8; 256] = &[
        // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0 0x00..=0x7F => 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
        0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C 0xC2..=0xDF => 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E 0xE0..=0xEF => 3,
        4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F 0xF0..=0xF4 => 4,
    ];

    /* public methods */

    /// Returns the expected UTF-8 byte length based on the given first byte, or `None` if invalid.
    ///
    /// LUT based (256-byte array).
    pub const fn utf8_len(self) -> Option<usize> {
        let width = self.utf8_len_unchecked();
        is![width == 0; None; Some(width)]
    }

    /// Returns the expected UTF-8 byte length based on the given first byte, or `0` if invalid.
    ///
    /// LUT based (256-byte array).
    pub const fn utf8_len_unchecked(self) -> usize {
        Self::UTF8_CHAR_LEN[self.0 as usize] as usize
    }

    /// Returns the expected UTF-8 byte length based on the given first byte, or `None` if invalid.
    ///
    /// Match based, for when memory accesses are more expensive than branches.
    #[must_use]
    pub const fn utf8_len_match(self) -> Option<usize> {
        match self.0 { // same logic as Self::UTF8_CHAR_LEN
            0x00..=0x7F => Some(1),
            0xC2..=0xDF => Some(2), // skips invalid C0, C1
            0xE0..=0xEF => Some(3),
            0xF0..=0xF4 => Some(4), // skips invalid 0xF5..0x=F7
            _ => None, // invalid leading byte
        }
    }

    /// Returns the expected UTF-8 byte length based on the given first byte.
    ///
    /// Match based, for when memory accesses are more expensive than branches.
    ///
    /// This function does **not** validate UTF-8 but determines how many bytes
    /// a valid sequence **should** occupy based on the leading byte.
    ///
    /// ### Caveat
    /// - If used on malformed UTF-8, it may suggest a length longer than the actual valid sequence.
    /// - Always use in conjunction with proper UTF-8 validation if handling untrusted input.
    pub const fn utf8_len_match_naive(self) -> usize {
        match self.0 {
            0x00..=0x7F => 1, // 1-byte ASCII
            0xC0..=0xDF => 2, // 2-byte sequence
            0xE0..=0xEF => 3, // 3-byte sequence
            0xF0..=0xF7 => 4, // 4-byte sequence
            _ => 0,           // invalid leading byte
        }
    }
}

/// # Methods over `u8` slice.
#[rustfmt::skip]
impl Char<&[u8]> {
    /// Decodes a UTF-8 code point at `index`.
    ///
    /// Returns `Some((code, len))` if the input is a valid UTF-8 sequence
    /// and the decoded code point is a valid Unicode scalar.
    ///
    /// Returns `None` if:
    /// - The index is out of bounds.
    /// - The bytes do not form a valid UTF-8 sequence.
    /// - The decoded value is not a valid Unicode scalar.
    ///
    /// It calls [to_code()][Self::to_code].
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Char;
    /// // Valid UTF-8 sequence
    /// let result = Char(b"\xE2\x82\xAC").to_char(0); // €
    /// assert_eq!(result, Some(('€', 3)));
    ///
    /// // Invalid continuation bytes
    /// let invalid_continuation = Char(b"\xE2\x41\xAC").to_char(0);
    /// assert_eq!(invalid_continuation, None);
    ///
    /// // Surrogate code point
    /// let surrogate = Char(b"\xED\xA0\x80").to_char(0); // U+D800
    /// assert_eq!(surrogate, None);
    ///
    /// // Out of bounds index
    /// let out_of_bounds = Char(b"hello").to_char(10);
    /// assert_eq!(out_of_bounds, None);
    ///
    /// // Incomplete sequence
    /// let incomplete = Char(b"\xE2\x82").to_char(0); // Missing third byte
    /// assert_eq!(incomplete, None);
    /// ```
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled, to avoid double validation.
    pub const fn to_char(self, index: usize) -> Option<(char, usize)> {
        let (cp, len) = unwrap![some? self.to_code(index)];

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return Some((unwrap![some? char::from_u32(cp)], len));

        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        Some((unsafe { char::from_u32_unchecked(cp) }, len))
    }

    /// Decodes a UTF-8 code point leniently at `index`, validating only the final Unicode scalar.
    ///
    /// This method is forgiving of UTF-8 encoding errors but ensures the result
    /// is a valid Unicode scalar value.
    ///
    /// Does not validate UTF-8 continuation bytes (may decode malformed sequences)
    ///
    /// It calls [to_code_unchecked()][Self::to_code_unchecked].
    ///
    /// # Panics
    /// Panics if the decoded code point is not a valid Unicode scalar value,
    /// or if the index is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Char;
    /// // Valid UTF-8 sequence
    /// let result = Char(b"\xE2\x82\xAC").to_char_lenient(0); // €
    /// assert_eq!(result, ('€', 3));
    ///
    /// // Invalid UTF-8 but decodes to valid scalar - behavior depends on input
    /// // This may return unexpected characters rather than panicking
    /// let result = Char(b"\xE2\x41\xAC").to_char_lenient(0);
    /// assert_eq!(result, ('\u{206c}', 3));
    ///
    /// // Surrogate code point - will panic
    /// // let result = Char(b"\xED\xA0\x80").to_char_lenient(0); // PANIC: U+D800 is invalid
    ///
    /// // Out of bounds index - will panic
    /// // let result = Char(b"hello").to_char_lenient(10); // PANIC: index out of bounds
    /// ```
    pub const fn to_char_lenient(self, index: usize) -> (char, usize) {
        let (cp, len) = self.to_code_unchecked(index);
        return (unwrap![some char::from_u32(cp)], len);
    }

    /// Decodes a UTF-8 code point at `index` without any validation.
    ///
    /// If the leading byte is invalid it returns the replacement character (`�`).
    ///
    /// It calls [`to_code_unchecked`][Self::to_code_unchecked].
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `index` is within bounds of `bytes`
    /// - `bytes[index..]` contains a valid UTF-8 sequence
    /// - The decoded code point is a valid Unicode scalar value
    ///
    /// Violating these conditions may lead to undefined behavior.
    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(not(base_safe_text), feature = "unsafe_str"))))]
    pub const unsafe fn to_char_unchecked(self, index: usize) -> (char, usize) {
        let (cp, len) = self.to_code_unchecked(index);
        (unsafe { char::from_u32_unchecked(cp) }, len)
    }

    /// Decodes a UTF-8 code point from the given byte slice, starting at `index`.
    ///
    /// Returns `Some((code, len))` if the input is a valid UTF-8 sequence
    /// and the decoded code point is a valid Unicode scalar.
    ///
    /// Returns `None` if:
    /// - The index is out of bounds.
    /// - The bytes do not form a valid UTF-8 sequence.
    /// - The decoded value is not a valid Unicode scalar.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Char;
    /// assert_eq!(Char("Ħ".as_bytes()).to_code(0), Some((u32::from('Ħ'), 2)));
    ///
    /// let invalid = b"\x80"; // Invalid leading byte
    /// assert_eq!(Char(invalid).to_code(0), None);
    /// ```
    #[must_use]
    pub const fn to_code(self, index: usize) -> Option<(u32, usize)> {
        let bytes = self.0;
        if index >= bytes.len() { return None; } // out of bounds
        let len = unwrap![some? Char(bytes[index]).utf8_len()]; // invalid leading byte
        if index + len > bytes.len() { return None; } // not enough bytes
        if !self.has_valid_continuation(index, len) { return None; } // malformed utf-8
        let (code, len) = Char(bytes).to_code_unchecked(index);
        is![Char(code).is_valid(); Some((code, len)); None] // invalid code point
    }

    /// Decodes a UTF-8 code point from `bytes`, starting at `index`.
    ///
    /// Returns `(code, len)`, where `code` is the decoded Unicode scalar,
    /// and `len` is the number of bytes consumed.
    ///
    /// It assumes `bytes[index..]` contains a valid UTF-8 sequence,
    /// and it doesn't validate the resulting unicode scalar.
    ///
    /// If the leading byte is invalid it returns the replacement character (`�`).
    ///
    /// # Panics
    /// It will panic if the index is out of bounds.
    #[must_use]
    pub const fn to_code_unchecked(self, index: usize) -> (u32, usize) {
        let bytes = self.0;
        let first = bytes[index];
        let len = Char(first).utf8_len_unchecked();
        if len == 0 { return (char::REPLACEMENT_CHARACTER as u32, 1); } // invalid leading byte
        let code = match len {
            1 => first as u32,
            2 => ((first as u32 & 0x1F) << 6) | (bytes[index + 1] as u32 & Char::<u32>::CONT_MASK),
            3 => ((first as u32 & 0x0F) << 12)
                | ((bytes[index + 1] as u32 & Char::<u32>::CONT_MASK) << 6)
                | (bytes[index + 2] as u32 & Char::<u32>::CONT_MASK),
            4 => ((first as u32 & 0x07) << 18)
                | ((bytes[index + 1] as u32 & Char::<u32>::CONT_MASK) << 12)
                | ((bytes[index + 2] as u32 & Char::<u32>::CONT_MASK) << 6)
                | (bytes[index + 3] as u32 & Char::<u32>::CONT_MASK),
            _ => char::REPLACEMENT_CHARACTER as u32,
        };
        (code, len)
    }

    /// Verifies that the continuation bytes following a UTF-8 leading byte are properly formatted.
    ///
    /// Each continuation byte must match the pattern `10xxxxxx` (i.e., have the high bits `0b10`).
    /// This ensures the byte sequence follows proper UTF-8 encoding rules.
    ///
    /// # Examples
    /// ```
    /// # use devela_base_core::Char;
    /// // euro sign €
    /// assert!(Char(b"\xE2\x82\xAC").has_valid_continuation(0, 3));
    ///
    /// // second byte is ASCII 'A', not continuation
    /// assert!(!Char(b"\xE2\x41\xAC").has_valid_continuation(0, 3));
    /// ```
    pub const fn has_valid_continuation(self, index: usize, len: usize) -> bool {
        let bytes = self.0;
        match len {
            1 => true, // no continuation bytes needed for ASCII
            2 => bytes[index + 1] & 0xC0 == 0x80,
            3 => bytes[index + 1] & 0xC0 == 0x80 && bytes[index + 2] & 0xC0 == 0x80,
            4 => bytes[index + 1] & 0xC0 == 0x80 && bytes[index + 2] & 0xC0 == 0x80
                && bytes[index + 3] & 0xC0 == 0x80,
            _ => false, // invalid length
        }
    }
}

/// Methods over a byte array, referring to a byte slice.
// (all wrapper methods are inlined away)
impl<const N: usize> Char<&[u8; N]> {
    /// A wrapper over [to_char()](#method.to_char).
    #[inline(always)]
    pub const fn to_char(self, index: usize) -> Option<(char, usize)> {
        let bytes: &[u8] = self.0;
        Char(bytes).to_char(index)
    }

    /// A wrapper over [to_char_lenient()](#method.to_char_lenient).
    #[inline(always)]
    pub const fn to_char_lenient(self, index: usize) -> (char, usize) {
        let bytes: &[u8] = self.0;
        Char(bytes).to_char_lenient(index)
    }

    /// A wrapper over [to_code()](#method.to_code).
    #[inline(always)]
    pub const fn to_code(self, index: usize) -> Option<(u32, usize)> {
        let bytes: &[u8] = self.0;
        Char(bytes).to_code(index)
    }

    /// A wrapper over [to_code_unchecked()](#method.to_code_unchecked).
    #[inline(always)]
    pub const fn to_code_unchecked(self, index: usize) -> (u32, usize) {
        let bytes: &[u8] = self.0;
        Char(bytes).to_code_unchecked(index)
    }

    /// A wrapper over [has_valid_continuation()](#method.has_valid_continuation).
    #[inline(always)]
    pub const fn has_valid_continuation(self, index: usize, len: usize) -> bool {
        let bytes: &[u8] = self.0;
        Char(bytes).has_valid_continuation(index, len)
    }
}
