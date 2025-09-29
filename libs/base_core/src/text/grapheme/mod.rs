// devela_base_core::text::grapheme
//
//! Extended Grapheme Clusters.
//

// mod _wip_machine; // WIP GraphemeMachine
mod nonul; // GraphemeNonul
mod u8; // GraphemeU8

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // _wip_machine::*, // WIP
            nonul::*,
            u8::*,
        };
    }
}
