// devela_base::text::grapheme
//
//! Extended Grapheme Clusters.
//

mod nonul; // GraphemeNonul
mod u8; // GraphemeU8

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            nonul::*,
            u8::*,
        };
    }
}
