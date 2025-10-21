// devela_base_core::text::char::scalar::utf8

use super::*;
use crate::{Char, CharAscii, DataOverflow, NonExtremeU32, NonNiche, Str, is, slice, unwrap};

/// Implements methods for both `char_utf8` and `char_utf8_niche`.
macro_rules! impl_char_utf8 {
    //
    () => {
        impl_char_utf8![@non char_utf8, NonNiche::<u32>, !char_utf8_niche];
        impl_char_utf8![@niche char_utf8_niche, NonExtremeU32, !char_utf8];
    };
    // specific implementations for the non-niche version
    (@non $name:ident, $inner:ty, !$other:ty) => { $crate::paste! {
        impl_char_utf8![@common $name, $inner, !$other];
        impl $name {
            #[doc = "Tries to convert this `" $name "` to `char7`."]
            ///
            /// # Errors
            /// Returns [`DataOverflow`] if `self` can't fit in 7 bits.
            #[inline(always)]
            pub const fn try_to_char7(self) -> Result<char7, DataOverflow> {
                char7::try_from_char_utf8(self)
            }
            #[doc = "Tries to convert this `" $name "` to `char8`."]
            ///
            /// # Errors
            /// Returns [`DataOverflow`] if `self` can't fit in 8 bits.
            #[inline(always)]
            pub const fn try_to_char8(self) -> Result<char8, DataOverflow> {
                char8::try_from_char_utf8(self)
            }
            #[doc = "Tries to convert this `" $name "` to `char17`."]
            ///
            /// # Errors
            /// Returns [`DataOverflow`] if `self` can't fit in 16 bits.
            #[inline(always)]
            pub const fn try_to_char16(self) -> Result<char16, DataOverflow> {
                char16::try_from_char_utf8(self)
            }
        }
    }};
    // specific implementations for the niche version
    (@niche $name:ident, $inner:ty, !$other:ty) => { $crate::paste! {
        impl_char_utf8![@common $name, $inner, !$other];
        // impl $name {}
    }};
    // common implementations for all versions
    (@common $name:ident, $inner:ty, !$other:ty) => { $crate::paste! {
        impl $name {
            /* private helper fns */

            const fn new_unchecked(value: u32) -> $name {
                #[cfg(any(base_safe_text, not(feature = "unsafe_niche")))]
                return $name(unwrap![some $inner::new(value)]);

                #[cfg(all(not(base_safe_text), feature = "unsafe_niche"))]
                unsafe {
                    $name($inner::new_unchecked(value))
                }
            }

            /* constants */

            #[doc = "The lowest Unicode scalar a `" $name "` can represent, `'\u{00}'`."]
            pub const MIN: $name = $name::new_unchecked(0x00);

            #[doc = "The highest Unicode scalar a `" $name "` can represent, `'\u{FF}'`."]
            pub const MAX: $name = $name::from_char(char::MAX);

            /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
            pub const REPLACEMENT_CHARACTER: $name =
                $name::from_char(char::REPLACEMENT_CHARACTER);

            /* from_* conversions */

            #[doc = "Creates a `" $name "` from a `char`."]
            pub const fn from_char(c: char) -> $name {
                Self::new_unchecked(u32::from_be_bytes(Char(c as u32).to_utf8_bytes_unchecked()))
            }

            #[doc = "Creates a `" $name "` from an `CharAscii`."]
            pub const fn from_char_ascii(c: CharAscii) -> $name {
                Self::new_unchecked(c as u8 as u32)
            }

            #[doc = "Creates a `" $name "` from a `char7`."]
            pub const fn from_char7(c: char7) -> $name {
                Self::new_unchecked(c.0.get() as u32)
            }

            #[doc = "Creates a `" $name "` from an `char8`."]
            pub const fn from_char8(c: char8) -> $name {
                Self::new_unchecked(
                    u32::from_be_bytes(Char(c.to_scalar()).to_utf8_bytes_unchecked()))
            }

            #[doc = "Creates a `" $name "` from an `char16`."]
            pub const fn from_char16(c: char16) -> $name {
                Self::new_unchecked(
                    u32::from_be_bytes(Char(c.to_scalar()).to_utf8_bytes_unchecked()))
            }

            #[doc = "Creates a `" $name "` from the first scalar value present in a string slice."]
            ///
            /// Returns `None` if the string is empty.
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::char_utf8;
            /// let c = char_utf8::from_str("A").unwrap();
            /// assert_eq!(c.to_utf8_bytes(), [b'A', 0, 0, 0]);
            ///
            /// let c = char_utf8::from_str("¢ rest").unwrap();
            /// assert_eq!(c.to_utf8_bytes(), [0xC2, 0xA2, 0, 0]);
            ///
            /// assert!(char_utf8::from_str("").is_none());
            /// ```
            ///
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            pub const fn from_str(string: &str) -> Option<$name> {
                is![string.is_empty(); None; Some($name::from_str_unchecked(string))]
            }

            #[doc = "Creates a `" $name "` from the first scalar value present in a string slice."]
            ///
            /// # Panics
            /// Panics if the string is empty.
            ///
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            pub const fn from_str_unchecked(string: &str) -> $name {
                debug_assert!(!string.is_empty(), "string must not be empty");
                let bytes = string.as_bytes();
                let len = Char(bytes[0]).len_utf8_unchecked();
                Self::decode_utf8(bytes, len)
            }

            #[doc = "Creates a `" $name "` from the first scalar value present in a string slice."]
            ///
            /// Returns the scalar and the bytes consumed as a `u32`.
            /// Returns `None` if the string is empty.
            ///
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            #[must_use]
            pub const fn from_str_with_len(string: &str) -> Option<($name, u32)> {
                is![string.is_empty(); None; Some($name::from_str_with_len_unchecked(string))]
            }

            #[doc = "Creates a `" $name "` from the first scalar value present in a string slice."]
            ///
            /// Returns the scalar and the bytes consumed as a `u32`.
            ///
            /// # Panics
            /// Panics if the string is empty.
            ///
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            pub const fn from_str_with_len_unchecked(string: &str) -> ($name, u32) {
                debug_assert!(!string.is_empty(), "string must not be empty");
                let bytes = string.as_bytes();
                let len = Char(bytes[0]).len_utf8_unchecked();
                (Self::decode_utf8(bytes, len), len as u32)
            }

            #[doc = "Creates a `" $name "` from the first UTF-8 encoded scalar value in a byte
            /// slice."]
            ///
            /// Returns `None` if the slice is empty or doesn't contain a valid UTF-8 sequence
            /// at the beginning.
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::char_utf8;
            /// let ascii = char_utf8::from_utf8_bytes(b"A").unwrap();
            /// assert_eq!(ascii.to_utf8_bytes(), [b'A', 0, 0, 0]);
            ///
            /// let multi_byte = char_utf8::from_utf8_bytes(b"\xC2\xA2 rest").unwrap(); // ¢
            /// assert_eq!(multi_byte.to_utf8_bytes(), [0xC2, 0xA2, 0, 0]);
            ///
            /// assert!(char_utf8::from_utf8_bytes(b"").is_none());
            /// assert!(char_utf8::from_utf8_bytes(b"\xC2").is_none()); // incomplete sequence
            /// ```
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            #[must_use] #[rustfmt::skip]
            pub const fn from_utf8_bytes(bytes: &[u8]) -> Option<$name> {
                is![bytes.is_empty(); return None];
                let len = unwrap![some? Char(bytes[0]).len_utf8()]; // invalid leading byte?
                is![!Char(bytes).has_valid_continuation(0, len); return None]; // malformed utf-8?
                is![Char(bytes).has_overlong_encoding(0, len); return None]; // overlong encoding?
                is![len == 3 && bytes[0] == 0xED && bytes[1] >= 0xA0; return None]; // surrogate?
                Some(Self::decode_utf8(bytes, len))
            }

            #[doc = "Creates a `" $name "` from the first UTF-8 encoded scalar value in a byte slice,
            /// without validation."]
            ///
            /// # Safety
            /// This function is unsafe because it does not check that:
            /// - The slice is non-empty.
            /// - The bytes form a valid UTF-8 sequence.
            /// - The decoded value is a valid Unicode scalar value.
            /// - The slice has enough bytes for the UTF-8 sequence.
            ///
            /// The caller must ensure the slice contains at least one complete, valid UTF-8
            /// encoded scalar value at the beginning.
            /// Violating these conditions may lead to undefined behavior.
            ///
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_utf8_bytes_unchecked(bytes: &[u8]) -> $name {
                let len = Char(bytes[0]).len_utf8_unchecked();
                Self::decode_utf8(bytes, len)
            }

            #[doc = "Creates a `" $name "` from the first UTF-8 encoded scalar value in a byte
            /// slice."]
            ///
            /// Returns the scalar and the consumed bytes.
            /// Returns `None` if the slice is empty
            /// or doesn't contain a valid UTF-8 sequence at the beginning.
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::char_utf8;
            /// let (c, len) = char_utf8::from_utf8_bytes_with_len(b"A").unwrap();
            /// assert!(c.to_utf8_bytes() == [b'A', 0, 0, 0] && len == 1);
            /// let (c, len) = char_utf8::from_utf8_bytes_with_len(b"\xC2\xA2 rest").unwrap(); // ¢
            /// assert!(c.to_utf8_bytes() == [0xC2, 0xA2, 0, 0] && len == 2);
            ///
            /// assert!(char_utf8::from_utf8_bytes_with_len(b"").is_none());
            /// assert!(char_utf8::from_utf8_bytes_with_len(b"\xC2").is_none()); // incomplete sequence
            /// ```
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            #[must_use] #[rustfmt::skip]
            pub const fn from_utf8_bytes_with_len(bytes: &[u8]) -> Option<($name, u32)> {
                is![bytes.is_empty(); return None];
                let len = unwrap![some? Char(bytes[0]).len_utf8()]; // invalid leading byte?
                is![!Char(bytes).has_valid_continuation(0, len); return None]; // malformed utf-8?
                is![Char(bytes).has_overlong_encoding(0, len); return None]; // overlong encoding?
                is![len == 3 && bytes[0] == 0xED && bytes[1] >= 0xA0; return None]; // surrogate?
                Some((Self::decode_utf8(bytes, len), len as u32))
            }

            #[doc = "Creates a `" $name "` from the first UTF-8 encoded scalar value in a byte slice,
            /// without validation."]
            ///
            /// # Safety
            /// This function is unsafe because it does not check that:
            /// - The slice is non-empty.
            /// - The bytes form a valid UTF-8 sequence.
            /// - The decoded value is a valid Unicode scalar value.
            /// - The slice has enough bytes for the UTF-8 sequence.
            ///
            /// The caller must ensure the slice contains at least one complete, valid UTF-8
            /// encoded scalar value at the beginning.
            /// Violating these conditions may lead to undefined behavior.
            ///
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_utf8_bytes_with_len_unchecked(bytes: &[u8]) -> ($name, u32) {
                let len = Char(bytes[0]).len_utf8_unchecked();
                (Self::decode_utf8(bytes, len), len as u32)
            }

            #[doc = "Creates a `" $name "` from an array of UTF-8 bytes."]
            ///
            /// This method validates that the bytes form a valid UTF-8 sequence.
            /// represent a valid Unicode scalar value.
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::char_utf8;
            /// assert!(char_utf8::from_utf8_byte_array([b'A', 0, 0, 0]).is_some());    // Valid
            /// assert!(char_utf8::from_utf8_byte_array([0xC2, 0xA2, 0, 0]).is_some()); // Valid (¢)
            /// assert!(char_utf8::from_utf8_byte_array([0xC0, 0x80, 0, 0]).is_none()); // overlong enc.
            /// assert!(char_utf8::from_utf8_byte_array([0xC2, 0x41, 0, 0]).is_none()); // malformed cont.
            /// ```
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            #[rustfmt::skip]
            pub const fn from_utf8_byte_array(bytes: [u8; 4]) -> Option<$name> {
                let len = unwrap![some? Char(bytes[0]).len_utf8()]; // invalid leading byte?
                is![!Char(&bytes).has_valid_continuation(0, len); return None]; // malformed utf-8?
                is![Char(&bytes).has_overlong_encoding(0, len); return None]; // overlong encoding?
                is![len == 3 && bytes[0] == 0xED && bytes[1] >= 0xA0; return None]; // surrogate?
                Some(Self::decode_utf8(&bytes, len))
            }

            #[doc = "Creates a `" $name "` from an array of UTF-8 bytes, without validation."]
            ///
            /// # Safety
            /// This function is unsafe because it does not check that the bytes form a valid:
            /// UTF-8 sequence or represent a valid Unicode scalar value. The caller must ensure:
            ///
            /// 1. The bytes form a valid UTF-8 encoded character.
            /// 2. The decoded value is a valid Unicode scalar (e.g. not a surrogate).
            /// 3. The array contains no overlong encodings.
            /// 4. For sequences shorter than 4 bytes, trailing bytes should be zero.
            ///
            /// Violating these conditions may lead to undefined behavior.
            ///
            /// # Features
            /// Uses the `unsafe_hint` feature to optimize out unreachable branches.
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
            pub const unsafe fn from_utf8_byte_array_unchecked(bytes: [u8; 4]) -> $name {
                let len = Char(bytes[0]).len_utf8_unchecked();
                Self::decode_utf8(&bytes, len)
            }

            // Private helper to create a char_utf8* from the first scalar value in a UTF-8 byte slice.
            // Most efficient implementation with jump table, bit shifting and oring.
            // This doesn't perform any validation and should be used carefully.
            #[inline(always)] #[rustfmt::skip]
            pub(crate) const fn decode_utf8(bytes: &[u8], len: usize) -> $name {
                let scalar = match len {
                    1 =>  (bytes[0] as u32) << 24,
                    2 => ((bytes[0] as u32) << 24)
                       | ((bytes[1] as u32) << 16),
                    3 => ((bytes[0] as u32) << 24)
                       | ((bytes[1] as u32) << 16)
                       | ((bytes[2] as u32) <<  8),
                    4 => ((bytes[0] as u32) << 24)
                       | ((bytes[1] as u32) << 16)
                       | ((bytes[2] as u32) <<  8)
                       |  (bytes[3] as u32),
                    _ => {
                        #[cfg(any(base_safe_text, not(feature = "unsafe_hint")))]
                        unreachable!();
                        #[cfg(all(not(base_safe_text), feature = "unsafe_hint"))]
                        unsafe {
                            ::core::hint::unreachable_unchecked()
                        }
                    }
                };
                Self::new_unchecked(scalar)
            }

            /* to_* conversions */

            #[doc = "Converts this `" $name "` to a `" $other "` representation."]
            ///
            #[inline(always)]
            pub const fn [<to_ $other>](self) -> $other {
                $other::new_unchecked(self.0.get())
            }

            #[doc = "Converts this `" $name "` to a `char` Unicode scalar."]
            ///
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[must_use]
            #[inline(always)]
            pub const fn to_char(self) -> char {
                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return unwrap![some char::from_u32(self.to_scalar())];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: char_utf8* always has valid UTF-8 in 0..len.
                return unsafe { char::from_u32_unchecked(self.to_scalar()) };
            }

            #[doc = "Converts this `" $name "` to a `u32` Unicode scalar value."]
            #[must_use]
            #[inline(always)]
            pub const fn to_scalar(self) -> u32 {
                Char(&self.to_utf8_bytes()).to_scalar_unchecked(0).0
            }

            /// Writes the UTF-8 representation to a buffer and returns it as a string slice.
            ///
            /// # Example
            /// ```
            #[doc = "# use devela_base_core::" $name ";"]
            #[doc = "let c = " $name "::from_char('A');"]
            /// let mut buf = [0u8; 4];
            /// assert_eq!(c.as_str_into(&mut buf), "A");
            ///
            #[doc = "let c = " $name "::from_char('¢');"]
            /// let mut buf = [0u8; 4];
            /// assert_eq!(c.as_str_into(&mut buf), "¢");
            /// ```
            /// # Features
            /// Uses the `unsafe_str` feature to avoid duplicated validation.
            #[must_use]
            pub const fn as_str_into<'a>(&self, buf: &'a mut [u8; 4]) -> &'a str {
                *buf = self.to_utf8_bytes();
                let len = self.len_utf8();

                #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
                return unwrap![ok Str::from_utf8(slice![mut buf, ..len])];

                #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
                // SAFETY: char_utf8 always contains valid UTF-8
                unsafe {
                    Str::from_utf8_unchecked(slice![mut buf, ..len])
                }
            }

            /// Copy UTF-8 bytes into caller buffer and return a slice of the valid bytes.
            #[must_use]
            #[inline(always)]
            pub const fn as_bytes_into<'a>(&self, buf: &'a mut [u8; 4]) -> &'a [u8] {
                *buf = self.to_utf8_bytes();
                slice![buf, ..self.len_utf8()]
            }

            /// Returns the UTF-8 byte representation as a 4-byte array.
            ///
            /// The bytes are arranged with the **first UTF-8 byte at index 0**,
            /// exactly matching how UTF-8 is stored in `&str` and `[u8]`.
            /// Unused bytes in the sequence are set to 0.
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::char_utf8;
            /// let c = char_utf8::from_char('A');
            /// assert_eq!(c.to_utf8_bytes(), [0x41, 0, 0, 0]);
            ///
            /// let c = char_utf8::from_char('¢');
            /// assert_eq!(c.to_utf8_bytes(), [0xC2, 0xA2, 0, 0]);
            ///
            /// let c = char_utf8::from_char('€');
            /// assert_eq!(c.to_utf8_bytes(), [0xE2, 0x82, 0xAC, 0]);
            ///
            /// let c = char_utf8::from_char('𝅘𝅥𝅮');
            /// assert_eq!(c.to_utf8_bytes(), [0xF0, 0x9D, 0x85, 0xA0]);
            /// ```
            #[must_use]
            #[inline(always)]
            pub const fn to_utf8_bytes(self) -> [u8; 4] {
                self.0.get().to_be_bytes()
            }

            /// Returns the first byte of the UTF-8 representation.
            ///
            /// # Example
            /// ```
            /// # use devela_base_core::char_utf8;
            /// let c = char_utf8::from_char('A');
            /// assert_eq!(c.first_utf8_byte(), 0x41);
            ///
            /// let c = char_utf8::from_char('¢');
            /// assert_eq!(c.first_utf8_byte(), 0xC2);
            ///
            /// let c = char_utf8::from_char('€');
            /// assert_eq!(c.first_utf8_byte(), 0xE2);
            /// ```
            #[must_use]
            #[inline(always)]
            pub const fn first_utf8_byte(self) -> u8 {
                (self.0.get() >> 24) as u8
            }

            /* queries */

            /// Returns `true` if this Unicode scalar is a [noncharacter][0].
            ///
            /// [0]: https://www.unicode.org/glossary/#noncharacter
            #[must_use]
            #[inline(always)]
            pub const fn is_noncharacter(self) -> bool {
                Char(self.0.get()).is_noncharacter()
            }

            /// Returns `true` if this Unicode scalar is an [abstract character][0].
            ///
            /// [0]: https://www.unicode.org/glossary/#abstract_character
            #[must_use]
            #[inline(always)]
            pub const fn is_character(self) -> bool {
                !self.is_noncharacter()
            }

            /// Checks if the value is within the ASCII range.
            #[must_use]
            #[inline(always)]
            pub const fn is_ascii(self) -> bool {
                self.0.get() <= 0x7F
            }

            /// Returns `true` if this is the nul character (`0x00`).
            #[must_use]
            #[inline(always)]
            pub const fn is_nul(self) -> bool {
                self.0.get() == 0x00
            }

            /// Returns the length of the UTF-8 representation.
            #[must_use]
            #[inline(always)]
            pub const fn len_bytes(self) -> usize {
                Char(self.to_scalar()).len_bytes() // we need to convert to a scalar
            }

            /// Returns the length of the UTF-8 representation.
            #[must_use]
            #[inline(always)]
            pub const fn len_utf8(self) -> usize {
                Char::UTF8_CHAR_LEN[self.first_utf8_byte() as usize] as usize
            }
        }
    }};
}
impl_char_utf8!();
