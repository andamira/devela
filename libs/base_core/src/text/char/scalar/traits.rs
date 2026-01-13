// devela_base_core::text::char::impls::traits
//
//! implementations of core traits
//

use crate::{
    Binary, CapacityMismatch, ConstInitCore, Debug, Display, FmtResult, Formatter, LowerHex, Octal,
    UpperHex, paste, text::char::*, unwrap,
};

/* Default, Display, Debug */

///
macro_rules! char_core_impls {
    () => {
        char_core_impls![
            char7 + Self(unwrap![some NonExtremeU8::new(0)]),
            char8 + Self(0),
            char16 + Self(unwrap![some NonSurrogateU16::new(0)]),
            charu + Self(unwrap![some NonNiche::<u32>::new(0)]),
            charu_niche + Self(unwrap![some NonExtremeU32::new(0)])
        ];
    };
    ($( $name:ident + $default:expr ),+ $(,)?) => {
        $( char_core_impls![@$name + $default]; )+
    };
    (@$name:ident + $default:expr) => { paste! {
        impl Default for super::$name {
            /// Returns the default value of `\x00` (nul character).
            fn default() -> Self { $default }
        }
        impl ConstInitCore for super::$name {
            /// It has the initial value of `\x00` (nul character).
            const INIT: Self = $default;
        }

        impl Display for super::$name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                write!(f, "{}", self.to_char())
            }
        }
        impl Debug for super::$name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                write!(f, "{:?}", self.to_char())
            }
        }
        impl Binary for super::$name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                Binary::fmt(&self.to_scalar(), f)
            }
        }
        impl LowerHex for super::$name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                LowerHex::fmt(&self.to_scalar(), f)
            }
        }
        impl UpperHex for super::$name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                UpperHex::fmt(&self.to_scalar(), f)
            }
        }
        impl Octal for super::$name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
                Octal::fmt(&self.to_scalar(), f)
            }
        }
    }};
}
#[rustfmt::skip]
char_core_impls!();

/// From char7
#[rustfmt::skip]
mod c7 {
    use super::*;

    impl From<char7> for char { fn from(c: char7) -> char { c.to_char() } }
    impl From<char7> for char8 { fn from(c: char7) -> char8 { c.to_char8() } }
    impl From<char7> for char16 { fn from(c: char7) -> char16 { c.to_char16() } }
    impl From<char7> for charu { fn from(c: char7) -> charu { c.to_charu() } }

    impl AsRef<str> for char7 { fn as_ref(&self) -> &str { self.to_str() } }
    // string comparsions
    impl PartialEq<str> for char7 { fn eq(&self, s: &str) -> bool { self.to_str() == s } } // RHS
    impl PartialEq<&str> for char7 { fn eq(&self, s: &&str) -> bool { self.to_str() == *s } }
    impl PartialEq<char7> for str { fn eq(&self, s: &char7) -> bool { self == s.to_str() } } // LHS
    impl PartialEq<char7> for &str { fn eq(&self, s: &char7) -> bool { *self == s.to_str() } }
}

/// From char8
#[rustfmt::skip]
mod c8 {
    use super::*;

    impl From<char8> for char { fn from(c: char8) -> char { c.to_char() } }
    impl From<char8> for char16 { fn from(c: char8) -> char16 { c.to_char16() } }
    impl From<char8> for charu { fn from(c: char8) -> charu { c.to_charu() } }

    impl TryFrom<char8> for char7 { type Error = CapacityMismatch;
        fn try_from(c: char8) -> Result<char7, CapacityMismatch> { c.try_to_char7() }
    }
    // string comparisons via conversion to charu
    impl PartialEq<str> for char8 { fn eq(&self, s: &str) -> bool { // RHS
        let mut buf = [0; 4]; self.to_charu().as_str_into(&mut buf) == s } }
    impl PartialEq<&str> for char8 { fn eq(&self, s: &&str) -> bool {
        let mut buf = [0; 4]; self.to_charu().as_str_into(&mut buf) == *s } }
    impl PartialEq<char8> for str { fn eq(&self, s: &char8) -> bool { // LHS
        let mut buf = [0; 4]; self == s.to_charu().as_str_into(&mut buf) } }
    impl PartialEq<char8> for &str { fn eq(&self, s: &char8) -> bool {
        let mut buf = [0; 4]; *self == s.to_charu().as_str_into(&mut buf) } }
}

/// From char16
#[rustfmt::skip]
mod c16 {
    use super::*;

    impl From<char16> for char { fn from(c: char16) -> char { c.to_char() } }
    impl From<char16> for charu { fn from(c: char16) -> charu { c.to_charu() } }

    impl TryFrom<char16> for char7 {
        type Error = CapacityMismatch;
        fn try_from(c: char16) -> Result<char7, CapacityMismatch> { c.try_to_char7() }
    }
    impl TryFrom<char16> for char8 {
        type Error = CapacityMismatch;
        fn try_from(c: char16) -> Result<char8, CapacityMismatch> { c.try_to_char8() }
    }

    // string comparisons via conversion to charu
    impl PartialEq<str> for char16 { fn eq(&self, s: &str) -> bool { // RHS
        let mut buf = [0; 4]; self.to_charu().as_str_into(&mut buf) == s } }
    impl PartialEq<&str> for char16 { fn eq(&self, s: &&str) -> bool {
        let mut buf = [0; 4]; self.to_charu().as_str_into(&mut buf) == *s } }
    impl PartialEq<char16> for str { fn eq(&self, s: &char16) -> bool { // LHS
        let mut buf = [0; 4]; self == s.to_charu().as_str_into(&mut buf) } }
    impl PartialEq<char16> for &str { fn eq(&self, s: &char16) -> bool {
        let mut buf = [0; 4]; *self == s.to_charu().as_str_into(&mut buf) } }
}

/// From charu
#[rustfmt::skip]
mod utf8 {
    use super::*;

    impl From<charu> for char { fn from(c: charu) -> char { c.to_char() } }

    impl TryFrom<charu> for char7 {
        type Error = CapacityMismatch;
        fn try_from(c: charu) -> Result<char7, CapacityMismatch> { c.try_to_char7() }
    }
    impl TryFrom<charu> for char8 {
        type Error = CapacityMismatch;
        fn try_from(c: charu) -> Result<char8, CapacityMismatch> { c.try_to_char8() }
    }
    impl TryFrom<charu> for char16 {
        type Error = CapacityMismatch;
        fn try_from(c: charu) -> Result<char16, CapacityMismatch> { c.try_to_char16() }
    }
    // string comparisons
    impl PartialEq<str> for charu { fn eq(&self, s: &str) -> bool { // RHS
        let mut buf = [0; 4]; self.as_str_into(&mut buf) == s } }
    impl PartialEq<&str> for charu { fn eq(&self, s: &&str) -> bool {
        let mut buf = [0; 4]; self.as_str_into(&mut buf) == *s } }
    impl PartialEq<charu> for str { fn eq(&self, s: &charu) -> bool { // LHS
        let mut buf = [0; 4]; self == s.as_str_into(&mut buf) } }
    impl PartialEq<charu> for &str { fn eq(&self, s: &charu) -> bool {
        let mut buf = [0; 4]; *self == s.as_str_into(&mut buf) } }
}

/// From char
#[rustfmt::skip]
mod c {
    use super::*;
    impl From<char> for charu { fn from(c: char) -> charu { charu::from_char(c) } }

    impl TryFrom<char> for char7 {
        type Error = CapacityMismatch;
        fn try_from(c: char) -> Result<char7, CapacityMismatch> { char7::try_from_char(c) }
    }
    impl TryFrom<char> for char8 {
        type Error = CapacityMismatch;
        fn try_from(c: char) -> Result<char8, CapacityMismatch> { char8::try_from_char(c) }
    }
    impl TryFrom<char> for char16 {
        type Error = CapacityMismatch;
        fn try_from(c: char) -> Result<char16, CapacityMismatch> { char16::try_from_char(c) }
    }
}
