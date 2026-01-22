// devela_base_core::text::char::iter::str

use crate::{Char, CharIter, char7, char8, char16, charu, is, slice};

/// Methods available when constructed from a string slice.
impl<'a> CharIter<'a, &str> {
    /* constructors */

    /// Returns a new iterator over the Unicode scalars of a `string` slice.
    #[inline(always)] #[rustfmt::skip]
    pub const fn new(string: &'a str) -> Self {
        Self::_new(string.as_bytes(), 0)
    }

    /// Returns a new iterator over the Unicode scalars of a `string` slice,
    /// starting at `index`.
    ///
    /// Returns `None` if the given index is not a valid character boundary.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn new_at(string: &'a str, index: usize) -> Option<Self> {
        if string.is_char_boundary(index) {
            Some(Self::_new(string.as_bytes(), index))
        } else {
            None
        }
    }

    /* misc. */

    /// Returns the total number of Unicode scalars, consuming the iterator.
    pub const fn count(mut self) -> usize {
        let mut counter = 0;
        while self.pos < self.bytes.len() {
            let (_, len) = Char(self.bytes).to_scalar_unchecked(self.pos);
            self.pos += len;
            counter += 1;
        }
        counter
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
    /// assert_eq![iter.next_charu().unwrap(), "C"]; // Basic Latin
    /// assert_eq![iter.next_charu().unwrap(), "Ð"]; // Latin-1 supplement
    /// assert_eq![iter.next_charu().unwrap(), "€"]; // Basic Multilingual plane
    /// assert_eq![iter.next_charu().unwrap(), "𐌅"]; // Supplementary Multilingual Plane
    /// assert_eq![iter.next_charu().unwrap(), "G"]; // Basic Latin
    /// assert![iter.next_charu().is_none()];
    /// ```
    /// # Features
    /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
    #[must_use] #[rustfmt::skip]
    pub const fn next_charu(&mut self) -> Option<charu> {
        is![self.pos >= self.bytes.len(); return None];
        let len = Char(self.bytes[self.pos]).len_utf8_unchecked();
        let ch = charu::decode_utf8(slice![self.bytes, self.pos,..], len);
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
