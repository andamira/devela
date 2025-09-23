// devela_base_core::text::char::impls::traits
//
//! implementations of core traits
//

use crate::{DataOverflow, paste, text::char::*, unwrap};
use ::core::fmt;

/* Default, Display, Debug */

///
macro_rules! char_core_impls {
    () => {
        char_core_impls![
            char7 + Self(unwrap![some NonExtremeU8::new(0)]),
            char8 + Self(0),
            char16 + Self(unwrap![some NonSurrogateU16::new(0)])
        ];
    };
    ($( $name:ident + $default:expr ),+ ) => {
        $(
            char_core_impls![@$name + $default];
        )+
    };
    (@$name:ident + $default:expr) => { paste! {
        impl Default for super::$name {
            /// Returns the default value of `\x00` (nul character).
            fn default() -> Self { $default }
        }
        // TEMP
        // impl ConstDefault for super::$name {
        //     /// Returns the default value of `\x00` (nul character).
        //     const DEFAULT: Self = $default;
        // }
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

mod c7 {
    use super::super::*;

    impl From<char7> for char {
        fn from(c: char7) -> char {
            c.to_char()
        }
    }
    impl From<char7> for char8 {
        fn from(c: char7) -> char8 {
            c.to_char8()
        }
    }
    impl From<char7> for char16 {
        fn from(c: char7) -> char16 {
            c.to_char16()
        }
    }
}

/* From char8 */

mod c8 {
    use super::*;

    impl From<char8> for char {
        fn from(c: char8) -> char {
            c.to_char()
        }
    }
    impl TryFrom<char8> for char7 {
        type Error = DataOverflow;
        fn try_from(c: char8) -> Result<char7, DataOverflow> {
            c.try_to_char7()
        }
    }
    impl From<char8> for char16 {
        fn from(c: char8) -> char16 {
            c.to_char16()
        }
    }
}

/* From char16 */

mod c16 {
    use super::*;

    impl From<char16> for char {
        fn from(c: char16) -> char {
            c.to_char()
        }
    }
    impl TryFrom<char16> for char7 {
        type Error = DataOverflow;
        fn try_from(c: char16) -> Result<char7, DataOverflow> {
            c.try_to_char7()
        }
    }
    impl TryFrom<char16> for char8 {
        type Error = DataOverflow;
        fn try_from(c: char16) -> Result<char8, DataOverflow> {
            c.try_to_char8()
        }
    }
}

/* From char */

impl TryFrom<char> for char7 {
    type Error = DataOverflow;
    fn try_from(c: char) -> Result<char7, DataOverflow> {
        char7::try_from_char(c)
    }
}
impl TryFrom<char> for char8 {
    type Error = DataOverflow;
    fn try_from(c: char) -> Result<char8, DataOverflow> {
        char8::try_from_char(c)
    }
}
impl TryFrom<char> for char16 {
    type Error = DataOverflow;
    fn try_from(c: char) -> Result<char16, DataOverflow> {
        char16::try_from_char(c)
    }
}
