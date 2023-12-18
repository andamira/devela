// devela::text::char::core_impls
//
//! implementations of core traits
//

use super::{
    Char16, Char24, Char32, Char7, Char8, CharConversionError, NonMaxU8, NonSurrogateU16, Result,
};
use crate::code::paste;
use core::fmt;

/* Default, Display, Debug */

macro_rules! core_impls {
    ($name:ident: $( $bits:literal + $default:expr ),+ ) => {
        $( core_impls![@$name: $bits + $default]; )+
    };
    (@$name:ident: $bits:literal + $default:expr) => { paste! {
        /// Returns the default value of `\x00` (nul character).
        impl Default for [<$name $bits>] {
            #[inline]
            #[must_use]
            fn default() -> Self { $default }
        }
        impl fmt::Display for [<$name $bits>] {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_char())
            }
        }
        impl fmt::Debug for [<$name $bits>] {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.to_char())
            }
        }
        impl fmt::Binary for [<$name $bits>] {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Binary::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::LowerHex for [<$name $bits>] {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::UpperHex for [<$name $bits>] {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::UpperHex::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::Octal for [<$name $bits>] {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Octal::fmt(&self.to_u32(), f)
            }
        }
    }};
}
core_impls![Char:
    7+Self(NonMaxU8::new(0).unwrap()),
    8+Self(0),
    16+Self(NonSurrogateU16::new(0).unwrap()),
    24+Self{ hi: NonMaxU8::new(0).unwrap(), mi: 0, lo: 0 },
    32+Self('\x00')
];

/* From Char7 */

impl From<Char7> for Char8 {
    #[inline]
    #[must_use]
    fn from(c: Char7) -> Char8 {
        c.to_char8()
    }
}
impl From<Char7> for Char16 {
    #[inline]
    #[must_use]
    fn from(c: Char7) -> Char16 {
        c.to_char16()
    }
}
impl From<Char7> for Char24 {
    #[inline]
    #[must_use]
    fn from(c: Char7) -> Char24 {
        c.to_char24()
    }
}
impl From<Char7> for Char32 {
    #[inline]
    #[must_use]
    fn from(c: Char7) -> Char32 {
        c.to_char32()
    }
}
impl From<Char7> for char {
    #[inline]
    #[must_use]
    fn from(c: Char7) -> char {
        c.to_char()
    }
}

/* From Char8 */

impl TryFrom<Char8> for Char7 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char8) -> Result<Char7> {
        c.try_to_char7()
    }
}
impl From<Char8> for Char16 {
    #[inline]
    #[must_use]
    fn from(c: Char8) -> Char16 {
        c.to_char16()
    }
}
impl From<Char8> for Char24 {
    #[inline]
    #[must_use]
    fn from(c: Char8) -> Char24 {
        c.to_char24()
    }
}
impl From<Char8> for Char32 {
    #[inline]
    #[must_use]
    fn from(c: Char8) -> Char32 {
        c.to_char32()
    }
}
impl From<Char8> for char {
    #[inline]
    #[must_use]
    fn from(c: Char8) -> char {
        c.to_char()
    }
}

/* From Char16 */

impl TryFrom<Char16> for Char7 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char16) -> Result<Char7> {
        c.try_to_char7()
    }
}
impl TryFrom<Char16> for Char8 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char16) -> Result<Char8> {
        c.try_to_char8()
    }
}
impl From<Char16> for Char24 {
    #[inline]
    #[must_use]
    fn from(c: Char16) -> Char24 {
        c.to_char24()
    }
}
impl From<Char16> for Char32 {
    #[inline]
    #[must_use]
    fn from(c: Char16) -> Char32 {
        #[cfg(not(feature = "unsafe_text"))]
        return c.to_char32();

        // SAFETY: we've already checked we contain a valid char.
        #[cfg(feature = "unsafe_text")]
        return unsafe { Char32(char::from_u32_unchecked(c.0.get() as u32)) };
    }
}
impl From<Char16> for char {
    #[inline]
    #[must_use]
    fn from(c: Char16) -> char {
        c.to_char()
    }
}

/* From Char24 */

impl TryFrom<Char24> for Char7 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char24) -> Result<Char7> {
        c.try_to_char7()
    }
}
impl TryFrom<Char24> for Char8 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char24) -> Result<Char8> {
        c.try_to_char8()
    }
}
impl TryFrom<Char24> for Char16 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char24) -> Result<Char16> {
        c.try_to_char16()
    }
}
impl From<Char24> for Char32 {
    #[inline]
    #[must_use]
    fn from(c: Char24) -> Char32 {
        #[cfg(not(feature = "unsafe_text"))]
        return c.to_char32();

        // SAFETY: we've already checked we contain a valid char.
        #[cfg(feature = "unsafe_text")]
        return unsafe { Char32(char::from_u32_unchecked(c.to_u32())) };
    }
}
impl From<Char24> for char {
    #[inline]
    #[must_use]
    fn from(c: Char24) -> char {
        c.to_char()
    }
}

/* From Char32 */

impl TryFrom<Char32> for Char7 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char32) -> Result<Char7> {
        c.try_to_char7()
    }
}
impl TryFrom<Char32> for Char8 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char32) -> Result<Char8> {
        c.try_to_char8()
    }
}
impl TryFrom<Char32> for Char16 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: Char32) -> Result<Char16> {
        c.try_to_char16()
    }
}
impl From<Char32> for Char24 {
    #[inline]
    #[must_use]
    fn from(c: Char32) -> Char24 {
        c.to_char24()
    }
}
impl From<Char32> for char {
    #[inline]
    #[must_use]
    fn from(c: Char32) -> char {
        c.to_char()
    }
}

/* From char */

impl TryFrom<char> for Char7 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: char) -> Result<Char7> {
        Char7::try_from_char(c)
    }
}
impl TryFrom<char> for Char8 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: char) -> Result<Char8> {
        Char8::try_from_char(c)
    }
}
impl TryFrom<char> for Char16 {
    type Error = CharConversionError;
    #[inline]
    fn try_from(c: char) -> Result<Char16> {
        Char16::try_from_char(c)
    }
}
impl From<char> for Char24 {
    #[inline]
    #[must_use]
    fn from(c: char) -> Char24 {
        Char24::from_char(c)
    }
}
impl From<char> for Char32 {
    #[inline]
    #[must_use]
    fn from(c: char) -> Char32 {
        Char32::from_char(c)
    }
}
