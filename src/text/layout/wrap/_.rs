// devela/src/text/layout/wrap/_.rs
//
//! Defines text wrapping types.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod iter; // TextBreakKind, TextLine, TextWrapIter
    mod segment; // TextSegment, TextSegmentKind
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            iter::*,
            segment::*,
        };
    }
}
