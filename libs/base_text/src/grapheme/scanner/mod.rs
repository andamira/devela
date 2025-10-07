// devela_base_text::grapheme::scanner
//
//! An scanner/segmentator of extended grapheme clusters.
//

mod machine; // GraphemeBoundary, GraphemeMachine
mod properties; // GraphemeProps, GraphemePropCb, GraphemePropInCb
mod scanner; // GraphemeScanner
mod trie;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            machine::{GraphemeBoundary, GraphemeMachine},
            properties::{GraphemePropCb, GraphemePropInCb, GraphemeProps},
            scanner::GraphemeScanner,
        };
    }
}
