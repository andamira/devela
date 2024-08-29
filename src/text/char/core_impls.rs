// devela::text::char::core_impls
//
//! implementations of core traits
//

#[allow(unused_imports)]
use crate::{
    _dep::_core::fmt,
    code::{paste, ConstDefault},
    error::unwrap,
    text::{char::*, TextError, TextResult as Result},
};

/* Default, Display, Debug */

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
            #[inline]
            #[must_use]
            fn default() -> Self { $default }
        }
        impl ConstDefault for super::$name {
            /// Returns the default value of `\x00` (nul character).
            const DEFAULT: Self = $default;
        }
        impl fmt::Display for super::$name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_char())
            }
        }
        impl fmt::Debug for super::$name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.to_char())
            }
        }
        impl fmt::Binary for super::$name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Binary::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::LowerHex for super::$name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::UpperHex for super::$name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::UpperHex::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::Octal for super::$name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Octal::fmt(&self.to_u32(), f)
            }
        }
    }};
}
#[rustfmt::skip]
core_impls![
    Char7 | "_char7" + Self(unwrap![some NonExtremeU8::new(0)]),
    Char8 | "_char8" + Self(0),
    Char16 | "_char16" + Self(unwrap![some NonSurrogateU16::new(0)]),
    Char24 | "_char24" + Self { hi: unwrap![some NonExtremeU8::new(0)], mi: 0, lo: 0 },
    Char32 | "_char32" + Self('\x00')
];

/* From Char7 */

#[cfg(feature = "_char7")]
mod char7 {
    use super::super::*;

    impl From<Char7> for char {
        #[inline]
        #[must_use]
        fn from(c: Char7) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char8")]
    impl From<Char7> for Char8 {
        #[inline]
        #[must_use]
        fn from(c: Char7) -> Char8 {
            c.to_char8()
        }
    }
    #[cfg(feature = "_char16")]
    impl From<Char7> for Char16 {
        #[inline]
        #[must_use]
        fn from(c: Char7) -> Char16 {
            c.to_char16()
        }
    }
    #[cfg(feature = "_char24")]
    impl From<Char7> for Char24 {
        #[inline]
        #[must_use]
        fn from(c: Char7) -> Char24 {
            c.to_char24()
        }
    }
    #[cfg(feature = "_char32")]
    impl From<Char7> for Char32 {
        #[inline]
        #[must_use]
        fn from(c: Char7) -> Char32 {
            c.to_char32()
        }
    }
}

/* From Char8 */

#[cfg(feature = "_char8")]
mod char8 {
    use super::*;

    impl From<Char8> for char {
        #[inline]
        #[must_use]
        fn from(c: Char8) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char7")]
    impl TryFrom<Char8> for Char7 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char8) -> Result<Char7> {
            c.try_to_char7()
        }
    }
    #[cfg(feature = "_char16")]
    impl From<Char8> for Char16 {
        #[inline]
        #[must_use]
        fn from(c: Char8) -> Char16 {
            c.to_char16()
        }
    }
    #[cfg(feature = "_char24")]
    impl From<Char8> for Char24 {
        #[inline]
        #[must_use]
        fn from(c: Char8) -> Char24 {
            c.to_char24()
        }
    }
    #[cfg(feature = "_char32")]
    impl From<Char8> for Char32 {
        #[inline]
        #[must_use]
        fn from(c: Char8) -> Char32 {
            c.to_char32()
        }
    }
}

/* From Char16 */

#[cfg(feature = "_char16")]
mod char16 {
    use super::*;

    impl From<Char16> for char {
        #[inline]
        #[must_use]
        fn from(c: Char16) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char7")]
    impl TryFrom<Char16> for Char7 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char16) -> Result<Char7> {
            c.try_to_char7()
        }
    }
    #[cfg(feature = "_char8")]
    impl TryFrom<Char16> for Char8 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char16) -> Result<Char8> {
            c.try_to_char8()
        }
    }
    #[cfg(feature = "_char24")]
    impl From<Char16> for Char24 {
        #[inline]
        #[must_use]
        fn from(c: Char16) -> Char24 {
            c.to_char24()
        }
    }
    #[cfg(feature = "_char32")]
    impl From<Char16> for Char32 {
        /// # Features
        /// Makes use of the `unsafe_str` feature if enabled.
        #[inline]
        #[must_use]
        fn from(c: Char16) -> Char32 {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            return c.to_char32();

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked we contain a valid char.
            return unsafe { Char32(char::from_u32_unchecked(c.0.get() as u32)) };
        }
    }
}

/* From Char24 */

#[cfg(feature = "_char24")]
mod char24 {
    use super::*;

    impl From<Char24> for char {
        #[inline]
        #[must_use]
        fn from(c: Char24) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char7")]
    impl TryFrom<Char24> for Char7 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char24) -> Result<Char7> {
            c.try_to_char7()
        }
    }
    #[cfg(feature = "_char8")]
    impl TryFrom<Char24> for Char8 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char24) -> Result<Char8> {
            c.try_to_char8()
        }
    }
    #[cfg(feature = "_char16")]
    impl TryFrom<Char24> for Char16 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char24) -> Result<Char16> {
            c.try_to_char16()
        }
    }
    #[cfg(feature = "_char32")]
    impl From<Char24> for Char32 {
        /// # Features
        /// Makes use of the `unsafe_str` feature if enabled.
        #[inline]
        #[must_use]
        fn from(c: Char24) -> Char32 {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            return c.to_char32();

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked we contain a valid char.
            return unsafe { Char32(char::from_u32_unchecked(c.to_u32())) };
        }
    }
}

/* From Char32 */

#[cfg(feature = "_char32")]
mod char32 {
    use super::*;

    impl From<Char32> for char {
        #[inline]
        #[must_use]
        fn from(c: Char32) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char7")]
    impl TryFrom<Char32> for Char7 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char32) -> Result<Char7> {
            c.try_to_char7()
        }
    }
    #[cfg(feature = "_char8")]
    impl TryFrom<Char32> for Char8 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char32) -> Result<Char8> {
            c.try_to_char8()
        }
    }
    #[cfg(feature = "_char16")]
    impl TryFrom<Char32> for Char16 {
        type Error = TextError;
        #[inline]
        fn try_from(c: Char32) -> Result<Char16> {
            c.try_to_char16()
        }
    }
    #[cfg(feature = "_char24")]
    impl From<Char32> for Char24 {
        #[inline]
        #[must_use]
        fn from(c: Char32) -> Char24 {
            c.to_char24()
        }
    }
}

/* From char */

#[cfg(feature = "_char7")]
impl TryFrom<char> for Char7 {
    type Error = TextError;
    #[inline]
    fn try_from(c: char) -> Result<Char7> {
        Char7::try_from_char(c)
    }
}
#[cfg(feature = "_char8")]
impl TryFrom<char> for Char8 {
    type Error = TextError;
    #[inline]
    fn try_from(c: char) -> Result<Char8> {
        Char8::try_from_char(c)
    }
}
#[cfg(feature = "_char16")]
impl TryFrom<char> for Char16 {
    type Error = TextError;
    #[inline]
    fn try_from(c: char) -> Result<Char16> {
        Char16::try_from_char(c)
    }
}
#[cfg(feature = "_char24")]
impl From<char> for Char24 {
    #[inline]
    #[must_use]
    fn from(c: char) -> Char24 {
        Char24::from_char(c)
    }
}
#[cfg(feature = "_char32")]
impl From<char> for Char32 {
    #[inline]
    #[must_use]
    fn from(c: char) -> Char32 {
        Char32::from_char(c)
    }
}
