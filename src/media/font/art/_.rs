// devela/src/media/font/art/_.rs
//
//! Ascii-art fonts.
//

crate::mods_in! {
    // #[cfg(test)]
    // mod _test;

    mod art; // FontArt
    mod block_4_3; // FONT_ART_BLOCK_4_3
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            art::FontArt,
            block_4_3::*,
        };
    }
}
