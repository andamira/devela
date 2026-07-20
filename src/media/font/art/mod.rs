// devela/src/media/font/art/mod.rs
//
//! Ascii-art fonts.
//

// #[cfg(test)]
// mod _test;

mod art; // FontArt
mod block_4_3; // FONT_ART_BLOCK_4_3

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            art::*,
            block_4_3::*,
        };
    }
}
