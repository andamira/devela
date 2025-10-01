// devela::text::char::chars
//
//! Defines [`Chars`] iterator.
//
// - methods over &str
// - methods over &[u8]
// - impl Iterator*

use crate::{Char, IteratorFused, PhantomData, char_utf8, char7, char8, char16, is, slice, unwrap};

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_ITERATOR!()]
/// An iterator over Unicode scalars.
#[doc = crate::_doc!(location_item: "text/char/struct.CharIter.html")]
///
/// Implements `Iterator<Item = char>` by default, but provides specialized methods
/// for other scalar types.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharIter<'a, Source> {
    bytes: &'a [u8],
    pos: usize,
    _source: PhantomData<Source>,
}

/// Methods available when constructed from a string slice.
impl<'a> CharIter<'a, &str> {
    /* constructors */

    /// Returns a new iterator over the Unicode scalars of a `string` slice.
    #[inline(always)] #[rustfmt::skip]
    pub const fn new(string: &'a str) -> Self {
        Self { bytes: string.as_bytes(), pos: 0, _source: PhantomData }
    }

    /// Returns a new iterator over the Unicode scalars of a `string` slice,
    /// starting at `index`.
    ///
    /// Returns `None` if the given index is not a valid character boundary.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn new_at(string: &'a str, index: usize) -> Option<Self> {
        if string.is_char_boundary(index) {
            Some(Self { bytes: string.as_bytes(), pos: index, _source: PhantomData })
        } else {
            None
        }
    }

    /* next_char* methods */

    /// Returns the next Unicode scalar.
    ///
    /// Returns `None` once there are no more characters left.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::CharIter;
    /// let input = "CÐ€𐌅G";
    /// let mut iter = CharIter::<&str>::new(input);
    /// assert_eq![iter.next_char().unwrap(), 'C']; // Basic Latin
    /// assert_eq![iter.next_char().unwrap(), 'Ð']; // Latin-1 supplement
    /// assert_eq![iter.next_char().unwrap(), '€']; // Basic Multilingual plane
    /// assert_eq![iter.next_char().unwrap(), '𐌅']; // Supplementary Multilingual Plane
    /// assert_eq![iter.next_char().unwrap(), 'G']; // Basic Latin
    /// assert![iter.next_char().is_none()];
    /// ```
    /// # Features
    /// Uses the `unsafe_str` feature to skip validation checks.
    #[must_use] #[rustfmt::skip]
    pub const fn next_char(&mut self) -> Option<char> {
        is![self.pos >= self.bytes.len(); return None];
        let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
        let ch = {
            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            { crate::unwrap![some? char::from_u32(cp)] }
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            unsafe { char::from_u32_unchecked(cp) }
        };
        self.pos += len;
        Some(ch)
    }

    /// Returns the next 7-bit Unicode scalar.
    ///
    /// Returns `None` once there are no more characters left,
    /// or if the next character is not ASCII.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::CharIter;
    /// let input = "CÐ€𐌅G";
    /// let mut iter = CharIter::<&str>::new(input);
    /// assert_eq![iter.next_char7().unwrap(), "C"]; // Basic Latin
    /// assert![iter.next_char7().is_none()];
    /// ```
    /// # Features
    /// Uses the `unsafe_niche` feature to skip validation checks.
    #[must_use]
    pub const fn next_char7(&mut self) -> Option<char7> {
        is![self.pos >= self.bytes.len(); return None];
        let byte = self.bytes[self.pos];
        is![byte.is_ascii(); { self.pos += 1; Some(char7::new_unchecked(byte)) }; None]
    }

    /// Returns the next 8-bit Unicode scalar.
    ///
    /// Returns `None` once there are no more characters left,
    /// or if the next character can't fit in 1 byte.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::CharIter;
    /// let input = "CÐ€𐌅G";
    /// let mut iter = CharIter::<&str>::new(input);
    /// assert_eq![iter.next_char8().unwrap(), "C"]; // Basic Latin
    /// assert_eq![iter.next_char8().unwrap(), "Ð"]; // Latin-1 supplement
    /// assert![iter.next_char8().is_none()];
    /// ```
    #[must_use]
    pub const fn next_char8(&mut self) -> Option<char8> {
        is![self.pos >= self.bytes.len(); return None];
        let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
        if Char(cp).len_bytes() == 1 {
            self.pos += len;
            Some(char8(cp as u8))
        } else {
            None
        }
    }

    /// Returns the next 16-bit Unicode scalar.
    ///
    /// Returns `None` once there are no more characters left,
    /// or if the next character can't fit in 2 bytes.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::CharIter;
    /// let input = "CÐ€𐌅G";
    /// let mut iter = CharIter::<&str>::new(input);
    /// assert_eq![iter.next_char16().unwrap(), "C"]; // Basic Latin
    /// assert_eq![iter.next_char16().unwrap(), "Ð"]; // Latin-1 supplement
    /// assert_eq![iter.next_char16().unwrap(), "€"]; // Basic Multilingual plane
    /// assert![iter.next_char16().is_none()];
    /// ```
    /// # Features
    /// Uses the `unsafe_niche` feature to skip validation checks.
    #[must_use]
    pub const fn next_char16(&mut self) -> Option<char16> {
        is![self.pos >= self.bytes.len(); return None];
        let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
        if Char(cp).len_bytes() <= 2 {
            self.pos += len;
            Some(char16::new_unchecked(cp as u16))
        } else {
            None
        }
    }

    /// Returns the next Unicode scalar using a UTF-8 representation.
    ///
    /// Returns `None` once there are no more characters left.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::CharIter;
    /// let input = "CÐ€𐌅G";
    /// let mut iter = CharIter::<&str>::new(input);
    /// assert_eq![iter.next_char_utf8().unwrap(), "C"]; // Basic Latin
    /// assert_eq![iter.next_char_utf8().unwrap(), "Ð"]; // Latin-1 supplement
    /// assert_eq![iter.next_char_utf8().unwrap(), "€"]; // Basic Multilingual plane
    /// assert_eq![iter.next_char_utf8().unwrap(), "𐌅"]; // Supplementary Multilingual Plane
    /// assert_eq![iter.next_char_utf8().unwrap(), "G"]; // Basic Latin
    /// assert![iter.next_char_utf8().is_none()];
    /// ```
    /// # Features
    /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
    #[must_use] #[rustfmt::skip]
    pub const fn next_char_utf8(&mut self) -> Option<char_utf8> {
        is![self.pos >= self.bytes.len(); return None];
        let len = Char(self.bytes[self.pos]).len_utf8_unchecked();
        let ch = char_utf8::decode_utf8(slice![self.bytes, self.pos,..], len);
        self.pos += len;
        Some(ch)
    }

    /* next_scalar* methods */

    /// Returns the next Unicode scalar value.
    ///
    /// This is implemented via `Char::`[`to_scalar_unchecked`][Char::to_scalar_unchecked].
    ///
    /// Returns `None` once there are no more characters left.
    #[must_use] #[rustfmt::skip]
    pub const fn next_scalar(&mut self) -> Option<u32> {
        is![self.pos >= self.bytes.len(); return None];
        let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
        self.pos += len;
        Some(cp)
    }

    /* find_* methods */

    /// Finds the next 7-bit ASCII character, skipping any non-ASCII bytes.
    ///
    /// Continues searching through the input until an ASCII character is found
    /// or the end is reached. Never returns `None` until the entire input is exhausted.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::CharIter;
    /// let input = "CÐ€𐌅G";
    /// let mut iter = CharIter::<&str>::new(input);
    /// assert_eq![iter.find_char7().unwrap(), "C"]; // Basic Latin
    /// assert_eq![iter.find_char7().unwrap(), "G"]; // Basic Latin
    /// ```
    /// # Features
    /// Uses the `unsafe_niche` feature to skip validation checks.
    #[must_use]
    pub const fn find_char7(&mut self) -> Option<char7> {
        while self.pos < self.bytes.len() {
            let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
            self.pos += len;
            is![Char(cp).is_ascii(); return Some(char7::new_unchecked(cp as u8))];
        }
        None
    }

    /// Finds the next Unicode scalar that fits in 8 bits, skipping larger scalars.
    ///
    /// Continues searching through the input until a scalar value ≤ 255 is found
    /// or the end is reached. Never returns `None` until the entire input is exhausted.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::CharIter;
    /// let input = "CÐ€𐌅G";
    /// let mut iter = CharIter::<&str>::new(input);
    /// assert_eq![iter.find_char8().unwrap(), "C"]; // Basic Latin
    /// assert_eq![iter.find_char8().unwrap(), "Ð"]; // Latin-1 supplement
    /// assert_eq![iter.find_char8().unwrap(), "G"]; // Basic Latin
    /// assert![iter.find_char8().is_none()];
    /// ```
    #[must_use]
    pub const fn find_char8(&mut self) -> Option<char8> {
        while self.pos < self.bytes.len() {
            let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
            self.pos += len;
            is![Char(cp).len_bytes() == 1; return Some(char8(cp as u8))];
        }
        None
    }

    /// Finds the next Unicode scalar that fits in 16 bits, skipping larger scalars.
    ///
    /// Continues searching through the input until a scalar value ≤ 65535 is found
    /// or the end is reached. Never returns `None` until the entire input is exhausted.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::CharIter;
    /// let input = "CÐ€𐌅G";
    /// let mut iter = CharIter::<&str>::new(input);
    /// assert_eq![iter.find_char16().unwrap(), "C"]; // Basic Latin
    /// assert_eq![iter.find_char16().unwrap(), "Ð"]; // Latin-1 supplement
    /// assert_eq![iter.find_char16().unwrap(), "€"]; // Basic Multilingual plane
    /// assert_eq![iter.find_char16().unwrap(), "G"]; // Basic Latin
    /// assert![iter.find_char16().is_none()];
    /// ```
    /// # Features
    /// Uses the `unsafe_niche` feature to skip validation checks.
    #[must_use]
    pub const fn find_char16(&mut self) -> Option<char16> {
        while self.pos < self.bytes.len() {
            let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
            self.pos += len;
            is![Char(cp).len_bytes() <= 2; return Some(char16::new_unchecked(cp as u16))];
        }
        None
    }
}

/// Methods available when constructed from a byte slice.
impl<'a> CharIter<'a, &[u8]> {
    /* constructors */

    /// Returns a new iterator over the Unicode scalars of a slice of `bytes`.
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0, _source: PhantomData }
    }

    /// Returns a new iterator over the Unicode scalars of a slice of `bytes`,
    /// starting at `index`.
    ///
    /// Returns `None` if the given index is not a valid character boundary.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn new_at(bytes: &'a [u8], index: usize) -> Option<Self> {
        if Char(bytes).is_utf8_boundary(index) {
            Some(Self { bytes, pos: index, _source: PhantomData })
        } else {
            None
        }
    }

    /* next_char* methods */

    /// Returns the next Unicode scalar.
    ///
    /// This is implemented via `Char::`[`to_char`][Char::to_char].
    ///
    /// # Features
    /// Uses the `unsafe_niche` feature to skip duplicated validation checks.
    #[must_use]
    pub const fn next_char(&mut self) -> Option<char> {
        is![self.pos >= self.bytes.len(); return None];
        let Some((ch, len)) = Char(self.bytes).to_char(self.pos) else { return None };
        self.pos += len;
        Some(ch)
    }

    /// Returns the next Unicode scalar, without performing full UTF-8 validation,
    /// but mostly the final Unicode scalar.
    ///
    /// If the leading byte is invalid it returns the replacement character (`�`).
    ///
    /// This is implemented via `Char::`[`to_char_lenient`][Char::to_char_lenient].
    #[must_use]
    pub const fn next_char_lenient(&mut self) -> Option<char> {
        is![self.pos >= self.bytes.len(); return None];
        let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
        is![let Some(ch) = char::from_u32(cp); { self.pos += len; Some(ch) }; None]
    }

    /// Returns the next Unicode scalar, without performing UTF-8 validation.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `index` is within bounds of `bytes`.
    /// - `bytes[index..]` contains a valid UTF-8 sequence.
    /// - The decoded value is a valid Unicode scalar.
    ///
    /// Violating these conditions may lead to undefined behavior.
    #[must_use]
    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(not(base_safe_text), feature = "unsafe_str"))))]
    pub const unsafe fn next_char_unchecked(&mut self) -> Option<char> {
        is![self.pos >= self.bytes.len(); return None];
        let (ch, len) = unsafe { Char(self.bytes).to_char_unchecked(self.pos) };
        self.pos += len;
        Some(ch)
    }

    /// Returns the next 7-bit Unicode scalar.
    ///
    /// Returns `None` once there are no more characters left,
    /// or if the next character is not ASCII.
    ///
    /// # Features
    /// Uses the `unsafe_niche` feature to skip validation checks.
    #[must_use]
    pub const fn next_char7(&mut self) -> Option<char7> {
        is![self.pos >= self.bytes.len(); return None];
        let byte = self.bytes[0];
        is![byte.is_ascii(); { self.pos += 1; Some(char7::new_unchecked(byte)) }; None]
    }

    /// Returns the next 8-bit Unicode scalar.
    ///
    /// Returns `None` once there are no more characters left,
    /// or if the next character can't fit in 1 byte.
    #[must_use]
    pub const fn next_char8(&mut self) -> Option<char8> {
        is![self.pos >= self.bytes.len(); return None];
        let Some((cp, len)) = Char(self.bytes).to_scalar(self.pos) else { return None };
        if Char(cp).len_bytes() == 1 {
            self.pos += len;
            Some(char8(cp as u8))
        } else {
            None
        }
    }

    /// Returns the next 8-bit Unicode scalar, without performing UTF-8 validation.
    ///
    /// Returns `None` once there are no more characters left,
    /// or if the next character can't fit in 1 byte.
    ///
    /// # Panics
    /// It will panic if the index is out of bounds.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `index` is within bounds of `bytes`.
    /// - `bytes[index..]` contains a valid UTF-8 sequence.
    /// - The decoded value is a valid Unicode scalar.
    ///
    /// Violating these conditions may lead to undefined behavior.
    #[must_use]
    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(not(base_safe_text), feature = "unsafe_str"))))]
    pub const unsafe fn next_char8_unchecked(&mut self) -> Option<char8> {
        let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
        is![Char(cp).len_bytes() == 1; { self.pos += len; Some(char8(cp as u8)) }; None]
    }

    /// Returns the next 16-bit Unicode scalar.
    ///
    /// Returns `None` once there are no more characters left,
    /// or if the next character can't fit in 2 bytes.
    ///
    /// # Features
    /// Uses the `unsafe_niche` feature to skip validation checks.
    #[must_use]
    pub const fn next_char16(&mut self) -> Option<char16> {
        is![self.pos >= self.bytes.len(); return None];
        let Some((cp, len)) = Char(self.bytes).to_scalar(self.pos) else { return None };
        if Char(cp).len_bytes() <= 2 {
            self.pos += len;
            Some(char16::new_unchecked(cp as u16))
        } else {
            None
        }
    }

    /// Returns the next 16-bit Unicode scalar, without performing UTF-8 validation.
    ///
    /// Returns `None` once there are no more characters left,
    /// or if the next character can't fit in 2 bytes.
    ///
    /// # Panics
    /// It will panic if the index is out of bounds.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `index` is within bounds of `bytes`.
    /// - `bytes[index..]` contains a valid UTF-8 sequence.
    /// - The decoded value is a valid Unicode scalar.
    ///
    /// Violating these conditions may lead to undefined behavior.
    ///
    /// # Features
    /// Uses the `unsafe_niche` feature to skip validation checks.
    #[must_use]
    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(not(base_safe_text), feature = "unsafe_str"))))]
    pub const unsafe fn next_char16_unchecked(&mut self) -> Option<char16> {
        let (cp, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
        if Char(cp).len_bytes() <= 2 {
            self.pos += len;
            Some(char16::new_unchecked(cp as u16))
        } else {
            None
        }
    }
    /// Returns the next Unicode scalar using a UTF-8 representation.
    ///
    /// Returns `None` once there are no more characters left.
    ///
    /// # Features
    /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
    #[must_use] #[rustfmt::skip]
    pub const fn next_char_utf8(&mut self) -> Option<char_utf8> {
        is![self.pos >= self.bytes.len(); return None];
        let (ch, len) = unwrap![some? char_utf8::from_utf8_bytes_with_len(self.bytes)];
        self.pos += len as usize;
        Some(ch)
    }

    /// Returns the next Unicode scalar, without performing UTF-8 validation.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `index` is within bounds of `bytes`.
    /// - `bytes[index..]` contains a valid UTF-8 sequence.
    /// - The decoded value is a valid Unicode scalar.
    ///
    /// Violating these conditions may lead to undefined behavior.
    ///
    /// # Features
    /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
    #[must_use]
    #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(not(base_safe_text), feature = "unsafe_str"))))]
    pub const unsafe fn next_char_utf8_unchecked(&mut self) -> Option<char_utf8> {
        is![self.pos >= self.bytes.len(); return None];
        let (ch, len) = unsafe { char_utf8::from_utf8_bytes_with_len_unchecked(self.bytes) };
        self.pos += len as usize;
        Some(ch)
    }

    /* next_scalar* methods */

    /// Returns the next Unicode scalar value.
    ///
    /// This is implemented via `Char::`[`to_scalar`][Char::to_scalar].
    ///
    /// # Features
    /// Uses the `unsafe_niche` feature to skip duplicated validation checks.
    #[must_use]
    pub const fn next_scalar(&mut self) -> Option<u32> {
        is![self.pos >= self.bytes.len(); return None];
        let Some((ch, len)) = Char(self.bytes).to_scalar(self.pos) else { return None };
        self.pos += len;
        Some(ch)
    }

    /// Returns the next Unicode scalar, without performing UTF-8 validation.
    ///
    /// This is implemented via `Char::`[`to_scalar_unchecked`][Char::to_scalar_unchecked].
    ///
    /// It assumes `bytes[index..]` contains a valid UTF-8 sequence,
    /// and it doesn't validate the resulting Unicode scalar.
    ///
    /// If the leading byte is invalid it returns the replacement character (`�`).
    ///
    /// # Panics
    /// It will panic if the index is out of bounds.
    #[must_use]
    pub const fn next_scalar_unchecked(&mut self) -> Option<u32> {
        is![self.pos >= self.bytes.len(); return None];
        let (ch, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
        self.pos += len;
        Some(ch)
    }
}

/* impl Iterator* */

impl<'a> Iterator for CharIter<'a, &'a str> {
    type Item = char;

    #[inline(always)]
    fn next(&mut self) -> Option<char> {
        self.next_char()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.bytes.len() - self.pos;
        (remaining.div_ceil(4), Some(remaining))
    }
}
impl<'a> IteratorFused for CharIter<'a, &'a str> {}

impl<'a> Iterator for CharIter<'a, &'a [u8]> {
    type Item = char;

    #[inline(always)]
    fn next(&mut self) -> Option<char> {
        self.next_char()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.bytes.len() - self.pos;
        (remaining.div_ceil(4), Some(remaining))
    }
}
impl<'a> IteratorFused for CharIter<'a, &'a [u8]> {}
