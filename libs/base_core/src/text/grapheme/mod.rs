// devela_base_core::text::grapheme
//
//! Extended Grapheme Clusters.
//

mod scanner; // Grapheme[Boundary|Machine|Prop[Cb|InCb|s]|Scanner]
mod nonul; // GraphemeNonul
mod u8; // GraphemeU8

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            scanner::_all::*,
            nonul::*,
            u8::*,
        };
    }
}
