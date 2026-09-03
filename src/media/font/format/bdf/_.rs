// devela/src/media/font/format/bdf/_.rs
//
//! Glyph Bitmap Distribution Format.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod_ _parse;

    mod error; // BdfError
    mod namespace; // Bdf
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            error::BdfError,
            namespace::Bdf,
        };
    }
}
