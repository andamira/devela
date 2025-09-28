// devela::text::char::impls
//
//!
//
// TOC
// - specific implementations
// - common implementations

#[allow(unused_imports)]
use super::*; // char*

/* specific implementations */

mod c;
mod c16;
mod c7;
mod c8;
mod utf8;

/* common implementations */

mod traits; // core traits

/// implements `UnicodeScalar` for custom char types.
macro_rules! impl_char {
    () => {
        impl_char![7, 8, 16];
        // impl_char![@char_utf8]; // TODO
    };
    ($( $bits:literal),+ ) => { $crate::paste! {
        $( impl_char!(@[<char $bits>]); )+
    }};
    (@$name:ident) => {

        /* impl traits */

        impl UnicodeScalar for $name { // TODO:IMPROVE avoid converting to char
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;

            /* encode */

            fn len_bytes(self) -> usize { self.len_bytes() }
            fn len_utf8(self) -> usize { self.len_utf8() }
            fn len_utf16(self) -> usize { self.len_utf16() }
            fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
                self.to_char().encode_utf8(dst)
            }
            fn to_utf8_bytes(self) -> [u8; 4] {
                let dyn_array = self.to_utf8_bytes();
                let mut array = [0u8; 4];
                for i in 0..dyn_array.len() {
                    array[i] = dyn_array[i];
                }
                array
            }
            fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
                self.to_char().encode_utf16(dst)
            }
            fn to_digit(self, radix: u32) -> Option<u32> { self.to_digit(radix) }
            fn to_ascii_uppercase(self) -> Self { self.to_ascii_uppercase() }
            fn to_ascii_lowercase(self) -> Self { self.to_ascii_lowercase() }

            /* queries */

            fn is_noncharacter(self) -> bool { self.is_noncharacter() }
            fn is_digit(self, radix: u32) -> bool { self.is_digit(radix) }
            //
            fn is_control(self) -> bool { self.to_char().is_control() }
            fn is_nul(self) -> bool { self.is_nul() }
            fn is_alphabetic(self) -> bool { self.to_char().is_alphabetic() }
            fn is_numeric(self) -> bool { self.to_char().is_numeric() }
            fn is_alphanumeric(self) -> bool { self.to_char().is_alphanumeric() }
            fn is_lowercase(self) -> bool { self.to_char().is_lowercase() }
            fn is_uppercase(self) -> bool { self.to_char().is_uppercase() }
            fn is_whitespace(self) -> bool { self.to_char().is_whitespace() }
            //
            fn is_ascii(self) -> bool { self.is_ascii() }
        }

        /* impl const fns */

        impl $name {
            /* encode */

            /// Returns the number of bytes needed to represent the scalar value.
            #[must_use]
            pub const fn len_bytes(self) -> usize { Char(self.to_scalar()).len_bytes() }

            /// Returns the number of bytes needed to encode in UTF-8.
            #[must_use]
            pub const fn len_utf8(self) -> usize { self.to_char().len_utf8() }

            /// Returns the number of bytes needed to encode in UTF-16.
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
            #[must_use]
            pub const fn to_digit(self, radix: u32) -> Option<u32> {
                self.to_char().to_digit(radix)
            }

            /* queries */

            /// Returns `true` if this is the nul character (`0x00`).
            #[must_use]
            pub const fn is_nul(self) -> bool { self.to_scalar() == 0 }

            /// Checks if the Unicode scalar is a digit in the given radix.
            ///
            /// See also [`to_digit`][Self#method.to_digit].
            #[must_use]
            pub const fn is_digit(self, radix: u32) -> bool {
                if let Some(_) = self.to_digit(radix) { true } else { false }
            }
        }
    };
}
impl_char!();
