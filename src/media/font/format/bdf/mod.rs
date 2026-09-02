// devela/src/media/font/format/bdf/mod.rs
//
//! Glyph Bitmap Distribution Format.
//

#[cfg(test)]
mod _test;

mod _parse;

mod error; // BdfError
mod namespace; // Bdf

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            error::*,
            namespace::*,
        };
    }
}
