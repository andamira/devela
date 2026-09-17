//
//! Recursive expansion over token trees.
//

#[cfg(doctest)]
crate::items! {
    mod _test_attr;
    mod _test_paste;
    mod _test_segment;
}

mod attr;
mod error; // PasteError, PasteResult
mod paste; // body_paste, Paste
mod segment; // PasteSegment

pub(crate) use {error::*, paste::*, segment::*};
