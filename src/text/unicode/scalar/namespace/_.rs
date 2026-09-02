// devela/src/text/unicode/scalar/namespace/_.rs
//
//! Defines the [`Char`] namespace.
//
// TOC
// - struct Char
// - methods over u16

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Char

    mod char; // Char<char>
    mod u16; // Char<u16>
    mod u32; // Char<u32>
    mod byte; // Char<u8>
    mod slice; // Char<&[u8] | &[u8; N]>
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            define::Char,
        };
    }
    _hidden {
        pub use super::u32::__unicode_scalar_write_utf8_at;
    }
}
