// devela::text::unicode::scalar::namespace::slice
//
// TOC
// - methods over &[u8]
// - methods over &[u8; N]

use crate::{Char, is, unwrap};

/// # Methods over `u8` slice.
#[rustfmt::skip]
impl Char<&[u8]> {
    /// Decodes a UTF-8 scalar at `index`.
    ///
    /// Returns `Some((char, len))` if the input is a valid UTF-8 sequence
    /// and the decoded value is a valid Unicode scalar.
    ///
    /// Returns `None` if:
    /// - The index is out of bounds.
    /// - The bytes do not form a valid UTF-8 sequence.
    /// - The decoded value is not a valid Unicode scalar.
    ///
    /// This is implemented via `Char::`[`to_scalar()`][Self::to_scalar].
    ///
    /// # Examples
    /// ```
    /// # use devela::Char;
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
    /// Uses the `unsafe_str` feature to skip duplicated validation checks.
    #[must_use]
    pub const fn to_char(self, index: usize) -> Option<(char, usize)> {
        let (cp, len) = unwrap![some? self.to_scalar(index)]; // check cp is a valid scalar
        cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
            Some((unsafe { char::from_u32_unchecked(cp) }, len)) // SAFETY: we just checked
        } _ => { Some((unwrap![some? char::from_u32(cp)], len)) }}
    }

    /// Decodes a UTF-8 scalar leniently at `index`, validating only the final Unicode scalar.
    ///
    /// This method is forgiving of UTF-8 encoding errors but ensures the result
    /// is a valid Unicode scalar value.
    ///
    /// - Does not validate UTF-8 continuation bytes (may decode malformed sequences).
    /// - If the leading byte is invalid it returns the replacement character (`�`).
    ///
    /// This is implemented via `Char::`[to_scalar_unchecked()][Self::to_scalar_unchecked].
    ///
    /// # Panics
    /// Panics if the decoded value is not a valid Unicode scalar value,
    /// or if the `index` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use devela::Char;
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
    #[must_use]
    pub const fn to_char_lenient(self, index: usize) -> (char, usize) {
        let (cp, len) = self.to_scalar_unchecked(index);
        (unwrap![some char::from_u32(cp)], len)
    }

    /// Decodes a UTF-8 scalar at `index` without any validation.
    ///
    /// If the leading byte is invalid it returns the replacement character (`�`).
    ///
    /// This is implemented via `Char::`[`to_scalar_unchecked`][Self::to_scalar_unchecked].
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `index` is within bounds of `bytes`
    /// - `bytes[index..]` contains a valid UTF-8 sequence
    /// - The decoded value is a valid Unicode scalar.
    ///
    /// Violating these conditions may lead to undefined behavior.
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))))]
    pub const unsafe fn to_char_unchecked(self, index: usize) -> (char, usize) {
        let (cp, len) = self.to_scalar_unchecked(index);
        (unsafe { char::from_u32_unchecked(cp) }, len)
    }

    /// Decodes a UTF-8 scalar from the given byte slice, starting at `index`.
    ///
    /// Returns `(scalar, len)`, where `scalar` is the decoded Unicode scalar,
    /// and `len` is the number of bytes consumed.
    ///
    /// Returns `None` if:
    /// - The index is out of bounds.
    /// - The bytes do not form a valid UTF-8 sequence.
    /// - The decoded value is not a valid Unicode scalar.
    ///
    /// # Examples
    /// ```
    /// # use devela::Char;
    /// assert_eq!(Char("Ħ".as_bytes()).to_scalar(0), Some((u32::from('Ħ'), 2)));
    ///
    /// let invalid = b"\x80"; // Invalid leading byte
    /// assert_eq!(Char(invalid).to_scalar(0), None);
    /// ```
    #[must_use]
    pub const fn to_scalar(self, index: usize) -> Option<(u32, usize)> {
        if index >= self.0.len() { return None; } // out of bounds
        let (bytes, first) = (self.0, self.0[index]);
        if first < 0x80 { return Some((first as u32, 1)); } // ASCII fast path
        let len = unwrap![some? Char(bytes[index]).len_utf8()]; // invalid leading byte?
        if index + len > bytes.len() { return None; } // not enough bytes?
        if !self.has_valid_continuation(index, len) { return None; } // malformed utf-8?
        if self.has_overlong_encoding(index, len) { return None; } // overlong encoding?
        let scalar = self.decode_scalar(index, len);
        is![Char(scalar).is_valid_scalar(), Some((scalar, len)), None] // invalid scalar?
    }

    /// Decodes a UTF-8 scalar from the given byte slice, starting at `index`, without validation.
    ///
    /// Returns `(scalar, len)`, where `scalar` is the decoded Unicode scalar,
    /// and `len` is the number of bytes consumed.
    ///
    /// It assumes `bytes[index..]` contains a valid UTF-8 sequence,
    /// and it doesn't validate the resulting Unicode scalar.
    ///
    /// If the leading byte is invalid it returns the replacement character (`�`).
    ///
    /// # Panics
    /// It will panic if the index is out of bounds.
    #[must_use]
    pub const fn to_scalar_unchecked(self, index: usize) -> (u32, usize) {
        let first = self.0[index];
        if first < 0x80 { return (first as u32, 1); } // ASCII fast path
        let len = Char(first).len_utf8_unchecked();
        if len == 0 { return (char::REPLACEMENT_CHARACTER as u32, 1); } // invalid leading byte?
        (self.decode_scalar(index, len), len)
    }

    #[must_use]
    #[inline(always)]
    const fn decode_scalar(self, index: usize, len: usize) -> u32 {
        let (bytes, first) = (self.0, self.0[index]);
        match len {
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
        }
    }

    /// Returns `true` if the UTF-8 sequence starting at `index` is overlong encoded.
    ///
    /// This method only checks for overlong encodings, but not other UTF-8 validity rules.
    /// It does not verify continuation byte patterns nor invalid scalar values.
    ///
    /// Overlong encodings use more bytes than necessary to represent a character,
    /// which is invalid in well-formed UTF-8.
    ///
    /// # Examples
    /// ```
    /// # use devela::Char;
    /// assert!(Char(b"\xE0\x80\x80").has_overlong_encoding(0, 3)); // overlong encoding
    /// assert!(!Char(b"\xE0\xA0\x80").has_overlong_encoding(0, 3)); // valid 3-byte sequence
    /// ```
    #[must_use] #[rustfmt::skip]
    pub const fn has_overlong_encoding(self, index: usize, len: usize) -> bool {
        let bytes = self.0;
        if index + len > bytes.len() { return false; }
        let first = bytes[index];
        match len {
            // should've been 1: C0, C1 are always overlong
            2 => { first == 0xC0 || first == 0xC1 }
            // E0 80..9F are overlong (should be 1-2 bytes)
            3 if first == 0xE0 => { let second = bytes[index + 1]; second < 0xA0 }
            // F0 80..8F are overlong (should be 1-3 bytes)
            4 if first == 0xF0 => { let second = bytes[index + 1]; second < 0x90 }
            _ => false, // 1-byte sequences can't be overlong
        }
    }

    /// Verifies that the continuation bytes following a UTF-8 leading byte are properly formatted.
    ///
    /// Each continuation byte must match the pattern `10xxxxxx` (i.e., have the high bits `0b10`).
    /// This ensures the byte sequence follows proper UTF-8 encoding rules.
    ///
    /// This method only verifies correct syntax, but not correct semantics.
    /// It does not check for overlong encodings nor invalid scalar values.
    ///
    /// # Examples
    /// ```
    /// # use devela::Char;
    /// assert!(Char(b"\xE2\x82\xAC").has_valid_continuation(0, 3)); // euro sign €
    /// assert!(!Char(b"\xE2\x41\xAC").has_valid_continuation(0, 3)); // second byte is ASCII 'A'
    /// assert!(!Char(b"\xC2").has_valid_continuation(0, 2)); // incomplete sequence
    /// ```
    pub const fn has_valid_continuation(self, index: usize, len: usize) -> bool {
        let bytes = self.0;
        is![bytes.len() < index + len, return false]; // ensure sufficient len
        match len {
            1 => true, // no continuation bytes needed for ASCII
            2 => bytes[index + 1] & 0xC0 == 0x80,
            3 => bytes[index + 1] & 0xC0 == 0x80 && bytes[index + 2] & 0xC0 == 0x80,
            4 => bytes[index + 1] & 0xC0 == 0x80 && bytes[index + 2] & 0xC0 == 0x80
                && bytes[index + 3] & 0xC0 == 0x80,
            _ => false, // invalid length
        }
    }

    /// Returns `true` if the byte at `index` is a valid starting point for a UTF-8 sequence.
    ///
    /// This checks if the byte is not a UTF-8 continuation byte (i.e., it's either
    /// an ASCII character or a valid leading byte of a multi-byte sequence).
    ///
    /// Useful for safely starting UTF-8 decoding from an arbitrary position in a byte slice.
    #[must_use]
    pub const fn is_utf8_boundary(self, index: usize) -> bool {
        index == self.0.len()
            || (index < self.0.len() && Char(self.0[index]).is_utf8_boundary())
    }
    /// Returns the smallest UTF-8 boundary `>= index`.
    #[must_use]
    pub const fn ceil_utf8_boundary(self, index: usize) -> usize {
        let bytes = self.0;
        let mut i = is![index < bytes.len(), index, bytes.len()];
        while i < bytes.len() && !Char(bytes).is_utf8_boundary(i) { i += 1; }
        i
    }
    /// Returns the greatest UTF-8 boundary `<= index`.
    ///
    /// If `index > self.len()`, starts from `self.len()`.
    ///
    /// This only checks boundary shape, not full UTF-8 validity.
    /// It is intended for already-valid UTF-8 byte slices.
    #[must_use]
    pub const fn floor_utf8_boundary(self, index: usize) -> usize {
        let bytes = self.0;
        let mut i = is![index < bytes.len(), index, bytes.len()];
        while i > 0 && !Char(bytes).is_utf8_boundary(i) { i -= 1; }
        i
    }
}

macro_rules! _impl_char_array_ref_wrappers {
    () => {};
    ($(#[$attr:meta])* unsafe fn $name:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty;
     $($rest:tt)* ) => {
        #[doc = concat!( "A wrapper over [`", stringify!($name), "()`](#method.", stringify!($name),
        ").", "\n\n# Safety\nSame requirements as the wrapped method.")]
        #[must_use] #[inline(always)] $(#[$attr])*
        pub const unsafe fn $name(self $(, $arg: $arg_ty)*) -> $ret {
            let bytes: &[u8] = self.0;
            unsafe { Char(bytes).$name($($arg),*) }
        }
        _impl_char_array_ref_wrappers!($($rest)*);
    };
    ($(#[$attr:meta])* fn $name:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty;
     $($rest:tt)* ) => {
        #[doc = concat!("A wrapper over [`",
            stringify!($name), "()`](#method.", stringify!($name), ").")]
        #[must_use] #[inline(always)] $(#[$attr])*
        pub const fn $name(self $(, $arg: $arg_ty)*) -> $ret {
            let bytes: &[u8] = self.0; Char(bytes).$name($($arg),*)
        }
        _impl_char_array_ref_wrappers!($($rest)*);
    };
}
/// Method wrappers over a byte array reference.
impl<const N: usize> Char<&[u8; N]> {
    _impl_char_array_ref_wrappers! {
        fn to_char(index: usize) -> Option<(char, usize)>;
        fn to_char_lenient(index: usize) -> (char, usize);
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        #[cfg_attr(nightly_doc, doc(cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))))]
        unsafe fn to_char_unchecked(index: usize) -> (char, usize);

        fn to_scalar(index: usize) -> Option<(u32, usize)>;
        fn to_scalar_unchecked(index: usize) -> (u32, usize);

        fn has_overlong_encoding(index: usize, len: usize) -> bool;
        fn has_valid_continuation(index: usize, len: usize) -> bool;

        fn is_utf8_boundary(index: usize) -> bool;
        fn ceil_utf8_boundary(index: usize) -> usize;
        fn floor_utf8_boundary(index: usize) -> usize;
    }
}
