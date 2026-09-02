// devela/src/text/unicode/grapheme/scanner/_.rs
//
//! An scanner/segmentator of extended grapheme clusters.
//

crate::mods_in! {
    mod iter; // GraphemeIter
    mod_ machine; // GraphemeBoundary, GraphemeMachine
    mod properties; // GraphemeProps, GraphemePropCb, GraphemePropInCb
    mod scanner; // GraphemeScanner

    mod trie;

    mod layout; // text-layout methods for GraphemeScanner
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            iter::GraphemeIter,
            machine::_all::{GraphemeBoundary, GraphemeMachine},
            properties::{GraphemePropCb, GraphemePropInCb, GraphemeProps},
            scanner::GraphemeScanner,
        };
    }
}
