// devela::text::char::impls
//
//!
//
// TOC
// - specific implementations
// - common implementations

#[allow(unused_imports)]
use super::*;

/* specific implementations */

mod char;
#[cfg(feature = "_char16")]
mod char16;
#[cfg(feature = "_char24")]
mod char24;
#[cfg(feature = "_char32")]
mod char32;
#[cfg(feature = "_char7")]
mod char7;
#[cfg(feature = "_char8")]
mod char8;

/* common implementations */

// implement UnicodeScalar for custom char types
macro_rules! impl_char {
    ($( $bits:literal | $feature:literal ),+ ) => { $(
        #[cfg(feature = $feature)]
        impl_char![@$bits];
    )+ };
    (@$bits:literal) => { crate::paste! {

        /* impl traits */

        impl UnicodeScalar for [<Char $bits>] {
            const MAX: Self = Self::MAX;

            /* encode */

            #[inline]
            fn byte_len(self) -> usize { self.byte_len() }
            #[inline]
            fn len_utf8(self) -> usize { self.len_utf8() }
            #[inline]
            fn len_utf16(self) -> usize { self.len_utf16() }
            #[inline]
            fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
                self.to_char().encode_utf8(dst)
            }
            #[inline]
            fn to_utf8_bytes(self) -> [u8; 4] {
                let dyn_array = self.to_utf8_bytes();
                let mut array = [0u8; 4];
                for i in 0..dyn_array.len() {
                    array[i] = dyn_array[i];
                }
                array
            }
            #[inline]
            fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
                self.to_char().encode_utf16(dst)
            }
            #[inline]
            fn to_digit(self, radix: u32) -> Option<u32> { self.to_digit(radix) }
            #[inline]
            fn to_ascii_uppercase(self) -> Self { self.to_ascii_uppercase() }
            #[inline]
            fn to_ascii_lowercase(self) -> Self { self.to_ascii_lowercase() }

            /* queries */

            #[inline]
            fn is_noncharacter(self) -> bool { self.is_noncharacter() }
            #[inline]
            fn is_digit(self, radix: u32) -> bool { self.is_digit(radix) }
            //
            #[inline]
            fn is_control(self) -> bool { self.to_char().is_control() }
            #[inline]
            fn is_nul(self) -> bool { self.is_nul() }
            #[inline]
            fn is_alphabetic(self) -> bool { self.to_char().is_alphabetic() }
            #[inline]
            fn is_numeric(self) -> bool { self.to_char().is_numeric() }
            #[inline]
            fn is_alphanumeric(self) -> bool { self.to_char().is_alphanumeric() }
            #[inline]
            fn is_lowercase(self) -> bool { self.to_char().is_lowercase() }
            #[inline]
            fn is_uppercase(self) -> bool { self.to_char().is_uppercase() }
            #[inline]
            fn is_whitespace(self) -> bool { self.to_char().is_whitespace() }
            //
            #[inline]
            fn is_ascii(self) -> bool { self.is_ascii() }
        }

        /* impl const fns */

        impl [<Char $bits>] {

            /* encode */

            /// Returns the number of bytes needed to represent the scalar value.
            #[inline]
            #[must_use]
            pub const fn byte_len(self) -> usize { char_byte_len(self.to_u32()) }

            /// Returns the number of bytes needed to encode in UTF-8.
            #[inline]
            #[must_use]
            pub const fn len_utf8(self) -> usize { self.to_char().len_utf8() }

            /// Returns the number of bytes needed to encode in UTF-16.
            #[inline]
            #[must_use]
            pub const fn len_utf16(self) -> usize { self.to_char().len_utf16() }

            /// Converts the scalar to a digit in the given radix.
            ///
            /// ‘Digit’ is defined to be only the following characters:
            /// `0-9`, `a-z`, `A-Z`.
            ///
            /// # Errors
            /// Returns None if the char does not refer to a digit in the given radix.
            ///
            /// # Panics
            /// Panics if given a radix larger than 36.
            #[inline]
            #[must_use]
            pub const fn to_digit(self, radix: u32) -> Option<u32> {
                self.to_char().to_digit(radix)
            }

            /* queries */

            /// Returns `true` if this is the nul character (`0x00`).
            #[inline]
            #[must_use]
            pub const fn is_nul(self) -> bool { self.to_u32() == 0 }

            /// Checks if the unicode scalar is a digit in the given radix.
            ///
            /// See also [`to_digit`][Self#method.to_digit].
            #[inline]
            #[must_use]
            pub const fn is_digit(self, radix: u32) -> bool {
                if let Some(_) = self.to_digit(radix) { true } else { false }
            }
        }
    }};
}
impl_char![7 | "_char7", 8 | "_char8", 16 | "_char16", 24 | "_char24", 32 | "_char32"];
