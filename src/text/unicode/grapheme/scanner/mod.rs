// devela/src/text/unicode/grapheme/scanner/mod.rs
//
//! An scanner/segmentator of extended grapheme clusters.
//

mod iter; // GraphemeIter
mod machine; // GraphemeBoundary, GraphemeMachine
mod properties; // GraphemeProps, GraphemePropCb, GraphemePropInCb
mod scanner; // GraphemeScanner

mod trie;

mod layout; // text-layout methods for GraphemeScanner

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            iter::GraphemeIter,
            machine::{GraphemeBoundary, GraphemeMachine},
            properties::{GraphemePropCb, GraphemePropInCb, GraphemeProps},
            scanner::GraphemeScanner,
        };
    }
}
