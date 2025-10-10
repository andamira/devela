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

mod c16;
mod c7;
mod c8;
mod utf8;

/* common implementations */

mod traits; // core traits

/// Implements const fns for custom char types.
macro_rules! impl_char {
    () => {
        impl_char![7, 8, 16];
    };
    ($( $bits:literal),+ ) => { $crate::paste! {
        $( impl_char!(@[<char $bits>]); )+
    }};
    (@$name:ident) => {
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
