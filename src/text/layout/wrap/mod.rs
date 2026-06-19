// devela/src/text/layout/wrap/mod.rs
//
//! Defines text wrapping types.
//

#[cfg(test)]
mod _tests;

mod iter; // TextBreakKind, TextLine, TextWrapIter
mod segment; // TextSegment, TextSegmentKind

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            iter::*,
            segment::*,
        };
    }
}
