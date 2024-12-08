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
///
macro_rules! char_core_impls {
    () => {
        char_core_impls![
            char7 | "_char7" + Self(unwrap![some NonExtremeU8::new(0)]),
            char8 | "_char8" + Self(0),
            char16 | "_char16" + Self(unwrap![some NonSurrogateU16::new(0)])
        ];
    };
    ($( $name:ident | $feature:literal + $default:expr ),+ ) => {
        $(
            #[cfg(feature = $feature)]
            char_core_impls![@$name + $default];
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
char_core_impls!();

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
