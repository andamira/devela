// devela/src/text/unicode/scalar/namespace/mod.rs
//
//! Defines the [`Char`] namespace.
//
// TOC
// - struct Char
// - methods over u16

#[cfg(test)]
mod tests;

mod char; // Char<char>
mod u16; // Char<u16>
mod u32; // Char<u32>
mod byte; // Char<u8>
mod slice; // Char<&[u8] | &[u8; N]>

mod define; // Char

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
