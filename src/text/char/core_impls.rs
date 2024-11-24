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
    char7 | "_char7" + Self(unwrap![some NonExtremeU8::new(0)]),
    char8 | "_char8" + Self(0),
    char16 | "_char16" + Self(unwrap![some NonSurrogateU16::new(0)]),
    char24 | "_char24" + Self { hi: unwrap![some NonExtremeU8::new(0)], mi: 0, lo: 0 },
    char32 | "_char32" + Self('\x00')
];

/* From char7 */

#[cfg(feature = "_char7")]
mod c7 {
    use super::super::*;

    impl From<char7> for char {
        #[must_use]
        fn from(c: char7) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char8")]
    impl From<char7> for char8 {
        #[must_use]
        fn from(c: char7) -> char8 {
            c.to_char8()
        }
    }
    #[cfg(feature = "_char16")]
    impl From<char7> for char16 {
        #[must_use]
        fn from(c: char7) -> char16 {
            c.to_char16()
        }
    }
    #[cfg(feature = "_char24")]
    impl From<char7> for char24 {
        #[must_use]
        fn from(c: char7) -> char24 {
            c.to_char24()
        }
    }
    #[cfg(feature = "_char32")]
    impl From<char7> for char32 {
        #[must_use]
        fn from(c: char7) -> char32 {
            c.to_char32()
        }
    }
}

/* From char8 */

#[cfg(feature = "_char8")]
mod c8 {
    use super::*;

    impl From<char8> for char {
        #[must_use]
        fn from(c: char8) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char7")]
    impl TryFrom<char8> for char7 {
        type Error = TextError;
        fn try_from(c: char8) -> Result<char7> {
            c.try_to_char7()
        }
    }
    #[cfg(feature = "_char16")]
    impl From<char8> for char16 {
        #[must_use]
        fn from(c: char8) -> char16 {
            c.to_char16()
        }
    }
    #[cfg(feature = "_char24")]
    impl From<char8> for char24 {
        #[must_use]
        fn from(c: char8) -> char24 {
            c.to_char24()
        }
    }
    #[cfg(feature = "_char32")]
    impl From<char8> for char32 {
        #[must_use]
        fn from(c: char8) -> char32 {
            c.to_char32()
        }
    }
}

/* From char16 */

#[cfg(feature = "_char16")]
mod c16 {
    use super::*;

    impl From<char16> for char {
        #[must_use]
        fn from(c: char16) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char7")]
    impl TryFrom<char16> for char7 {
        type Error = TextError;
        fn try_from(c: char16) -> Result<char7> {
            c.try_to_char7()
        }
    }
    #[cfg(feature = "_char8")]
    impl TryFrom<char16> for char8 {
        type Error = TextError;
        fn try_from(c: char16) -> Result<char8> {
            c.try_to_char8()
        }
    }
    #[cfg(feature = "_char24")]
    impl From<char16> for char24 {
        #[must_use]
        fn from(c: char16) -> char24 {
            c.to_char24()
        }
    }
    #[cfg(feature = "_char32")]
    impl From<char16> for char32 {
        /// # Features
        /// Makes use of the `unsafe_str` feature if enabled.
        #[must_use]
        fn from(c: char16) -> char32 {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            return c.to_char32();

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked we contain a valid char.
            return unsafe { char32(char::from_u32_unchecked(c.0.get() as u32)) };
        }
    }
}

/* From char24 */

#[cfg(feature = "_char24")]
mod c24 {
    use super::*;

    impl From<char24> for char {
        #[must_use]
        fn from(c: char24) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char7")]
    impl TryFrom<char24> for char7 {
        type Error = TextError;
        fn try_from(c: char24) -> Result<char7> {
            c.try_to_char7()
        }
    }
    #[cfg(feature = "_char8")]
    impl TryFrom<char24> for char8 {
        type Error = TextError;
        fn try_from(c: char24) -> Result<char8> {
            c.try_to_char8()
        }
    }
    #[cfg(feature = "_char16")]
    impl TryFrom<char24> for char16 {
        type Error = TextError;
        fn try_from(c: char24) -> Result<char16> {
            c.try_to_char16()
        }
    }
    #[cfg(feature = "_char32")]
    impl From<char24> for char32 {
        /// # Features
        /// Makes use of the `unsafe_str` feature if enabled.
        #[must_use]
        fn from(c: char24) -> char32 {
            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            return c.to_char32();

            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: we've already checked we contain a valid char.
            return unsafe { char32(char::from_u32_unchecked(c.to_u32())) };
        }
    }
}

/* From char32 */

#[cfg(feature = "_char32")]
mod c32 {
    use super::*;

    impl From<char32> for char {
        #[must_use]
        fn from(c: char32) -> char {
            c.to_char()
        }
    }
    #[cfg(feature = "_char7")]
    impl TryFrom<char32> for char7 {
        type Error = TextError;
        fn try_from(c: char32) -> Result<char7> {
            c.try_to_char7()
        }
    }
    #[cfg(feature = "_char8")]
    impl TryFrom<char32> for char8 {
        type Error = TextError;
        fn try_from(c: char32) -> Result<char8> {
            c.try_to_char8()
        }
    }
    #[cfg(feature = "_char16")]
    impl TryFrom<char32> for char16 {
        type Error = TextError;
        fn try_from(c: char32) -> Result<char16> {
            c.try_to_char16()
        }
    }
    #[cfg(feature = "_char24")]
    impl From<char32> for char24 {
        #[must_use]
        fn from(c: char32) -> char24 {
            c.to_char24()
        }
    }
}

/* From char */

#[cfg(feature = "_char7")]
impl TryFrom<char> for char7 {
    type Error = TextError;
    fn try_from(c: char) -> Result<char7> {
        char7::try_from_char(c)
    }
}
#[cfg(feature = "_char8")]
impl TryFrom<char> for char8 {
    type Error = TextError;
    fn try_from(c: char) -> Result<char8> {
        char8::try_from_char(c)
    }
}
#[cfg(feature = "_char16")]
impl TryFrom<char> for char16 {
    type Error = TextError;
    fn try_from(c: char) -> Result<char16> {
        char16::try_from_char(c)
    }
}
#[cfg(feature = "_char24")]
impl From<char> for char24 {
    #[must_use]
    fn from(c: char) -> char24 {
        char24::from_char(c)
    }
}
#[cfg(feature = "_char32")]
impl From<char> for char32 {
    #[must_use]
    fn from(c: char) -> char32 {
        char32::from_char(c)
    }
}
