// devela_macros::bodies::paste
//
//! Recursive expansion over token trees.
//

#[cfg(doctest)]
crate::items! {
    mod tests_attr;
    mod tests_paste;
    mod tests_segment;
}

mod attr;
mod error; // PasteError, PasteResult
mod paste; // body_paste, Paste
mod segment; // PasteSegment

pub(crate) use {error::*, paste::*, segment::*};
