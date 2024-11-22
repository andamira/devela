// devela::text::char::core_impls
//
//! implementations of core traits
//

#[allow(unused_imports)]
use crate::{
    _core::fmt, paste, text::char::*, unwrap, ConstDefault, TextError, TextResult as Result,
};

/* Default, Display, Debug */

#[doc = crate::doc_private!()]
macro_rules! core_impls {
    ($( $name:ident | $feature:literal + $default:expr ),+ ) => {
        $(
            #[cfg(feature = $feature)]
            core_impls![@$name + $default];
        )+
    };
    (@$name:ident + $default:expr) => { paste! {
        impl Default for super::$name {
            /// Returns the default value of `\x00` (nul character).
            #[must_use]
            fn default() -> Self { $default }
        }
        impl ConstDefault for super::$name {
            /// Returns the default value of `\x00` (nul character).
            const DEFAULT: Self = $default;
        }
        impl fmt::Display for super::$name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_char())
            }
        }
        impl fmt::Debug for super::$name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.to_char())
            }
        }
        impl fmt::Binary for super::$name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Binary::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::LowerHex for super::$name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::UpperHex for super::$name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::UpperHex::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::Octal for super::$name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Octal::fmt(&self.to_u32(), f)
            }
        }
    }};
}
#[rustfmt::skip]
core_impls![
    CharU7 | "_char_u7" + Self(unwrap![some NonExtremeU8::new(0)]),
    CharU8 | "_char_u8" + Self(0),
    CharU16 | "_char_u16" + Self(unwrap![some NonSurrogateU16::new(0)]),
    CharU24 | "_char_u24" + Self { hi: unwrap![some NonExtremeU8::new(0)], mi: 0, lo: 0 },
    CharU32 | "_char_u32" + Self('\x00')
];

/* From CharU7 */

#[cfg(feature = "_char_u7")]
mod char7 {
    use super::super::*;

    impl From<CharU7> for char {
        #[must_use]
        fn from(c: CharU7) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char_u8")]
    impl From<CharU7> for CharU8 {
        #[must_use]
        fn from(c: CharU7) -> CharU8 {
            c.to_char_u8()
        }
    }
    #[cfg(feature = "_char_u16")]
    impl From<CharU7> for CharU16 {
        #[must_use]
        fn from(c: CharU7) -> CharU16 {
            c.to_char_u16()
        }
    }
    #[cfg(feature = "_char_u24")]
    impl From<CharU7> for CharU24 {
        #[must_use]
        fn from(c: CharU7) -> CharU24 {
            c.to_char_u24()
        }
    }
    #[cfg(feature = "_char_u32")]
    impl From<CharU7> for CharU32 {
        #[must_use]
        fn from(c: CharU7) -> CharU32 {
            c.to_char_u32()
        }
    }
}

/* From CharU8 */

#[cfg(feature = "_char_u8")]
mod char8 {
    use super::*;

    impl From<CharU8> for char {
        #[must_use]
        fn from(c: CharU8) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char_u7")]
    impl TryFrom<CharU8> for CharU7 {
        type Error = TextError;
        fn try_from(c: CharU8) -> Result<CharU7> {
            c.try_to_char_u7()
        }
    }
    #[cfg(feature = "_char_u16")]
    impl From<CharU8> for CharU16 {
        #[must_use]
        fn from(c: CharU8) -> CharU16 {
            c.to_char_u16()
        }
    }
    #[cfg(feature = "_char_u24")]
    impl From<CharU8> for CharU24 {
        #[must_use]
        fn from(c: CharU8) -> CharU24 {
            c.to_char_u24()
        }
    }
    #[cfg(feature = "_char_u32")]
    impl From<CharU8> for CharU32 {
        #[must_use]
        fn from(c: CharU8) -> CharU32 {
            c.to_char_u32()
        }
    }
}

/* From CharU16 */

#[cfg(feature = "_char_u16")]
mod char16 {
    use super::*;

    impl From<CharU16> for char {
        #[must_use]
        fn from(c: CharU16) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char_u7")]
    impl TryFrom<CharU16> for CharU7 {
        type Error = TextError;
        fn try_from(c: CharU16) -> Result<CharU7> {
            c.try_to_char_u7()
        }
    }
    #[cfg(feature = "_char_u8")]
    impl TryFrom<CharU16> for CharU8 {
        type Error = TextError;
        fn try_from(c: CharU16) -> Result<CharU8> {
            c.try_to_char_u8()
        }
    }
    #[cfg(feature = "_char_u24")]
    impl From<CharU16> for CharU24 {
        #[must_use]
        fn from(c: CharU16) -> CharU24 {
            c.to_char_u24()
        }
    }
    #[cfg(feature = "_char_u32")]
    impl From<CharU16> for CharU32 {
        /// # Features
        /// Makes use of the `unsafe_str` feature if enabled.
        #[must_use]
        fn from(c: CharU16) -> CharU32 {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            return c.to_char_u32();

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked we contain a valid char.
            return unsafe { CharU32(char::from_u32_unchecked(c.0.get() as u32)) };
        }
    }
}

/* From CharU24 */

#[cfg(feature = "_char_u24")]
mod char24 {
    use super::*;

    impl From<CharU24> for char {
        #[must_use]
        fn from(c: CharU24) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char_u7")]
    impl TryFrom<CharU24> for CharU7 {
        type Error = TextError;
        fn try_from(c: CharU24) -> Result<CharU7> {
            c.try_to_char_u7()
        }
    }
    #[cfg(feature = "_char_u8")]
    impl TryFrom<CharU24> for CharU8 {
        type Error = TextError;
        fn try_from(c: CharU24) -> Result<CharU8> {
            c.try_to_char_u8()
        }
    }
    #[cfg(feature = "_char_u16")]
    impl TryFrom<CharU24> for CharU16 {
        type Error = TextError;
        fn try_from(c: CharU24) -> Result<CharU16> {
            c.try_to_char_u16()
        }
    }
    #[cfg(feature = "_char_u32")]
    impl From<CharU24> for CharU32 {
        /// # Features
        /// Makes use of the `unsafe_str` feature if enabled.
        #[must_use]
        fn from(c: CharU24) -> CharU32 {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            return c.to_char_u32();

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked we contain a valid char.
            return unsafe { CharU32(char::from_u32_unchecked(c.to_u32())) };
        }
    }
}

/* From CharU32 */

#[cfg(feature = "_char_u32")]
mod char32 {
    use super::*;

    impl From<CharU32> for char {
        #[must_use]
        fn from(c: CharU32) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char_u7")]
    impl TryFrom<CharU32> for CharU7 {
        type Error = TextError;
        fn try_from(c: CharU32) -> Result<CharU7> {
            c.try_to_char_u7()
        }
    }
    #[cfg(feature = "_char_u8")]
    impl TryFrom<CharU32> for CharU8 {
        type Error = TextError;
        fn try_from(c: CharU32) -> Result<CharU8> {
            c.try_to_char_u8()
        }
    }
    #[cfg(feature = "_char_u16")]
    impl TryFrom<CharU32> for CharU16 {
        type Error = TextError;
        fn try_from(c: CharU32) -> Result<CharU16> {
            c.try_to_char_u16()
        }
    }
    #[cfg(feature = "_char_u24")]
    impl From<CharU32> for CharU24 {
        #[must_use]
        fn from(c: CharU32) -> CharU24 {
            c.to_char_u24()
        }
    }
}

/* From char */

#[cfg(feature = "_char_u7")]
impl TryFrom<char> for CharU7 {
    type Error = TextError;
    fn try_from(c: char) -> Result<CharU7> {
        CharU7::try_from_char(c)
    }
}
#[cfg(feature = "_char_u8")]
impl TryFrom<char> for CharU8 {
    type Error = TextError;
    fn try_from(c: char) -> Result<CharU8> {
        CharU8::try_from_char(c)
    }
}
#[cfg(feature = "_char_u16")]
impl TryFrom<char> for CharU16 {
    type Error = TextError;
    fn try_from(c: char) -> Result<CharU16> {
        CharU16::try_from_char(c)
    }
}
#[cfg(feature = "_char_u24")]
impl From<char> for CharU24 {
    #[must_use]
    fn from(c: char) -> CharU24 {
        CharU24::from_char(c)
    }
}
#[cfg(feature = "_char_u32")]
impl From<char> for CharU32 {
    #[must_use]
    fn from(c: char) -> CharU32 {
        CharU32::from_char(c)
    }
}
