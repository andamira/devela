// devela::text::grapheme::trait
//
//!
//

use crate::{char, char_utf8, char7, char8, char16};

#[doc = crate::_TAG_TEXT!()]
/// Common trait for <abbr title="Extended Grapheme Cluster">EGC</abbr> types.
pub trait Grapheme {}

#[cfg(feature = "text")]
crate::items! {
    impl<const CAP: usize> Grapheme for crate::GraphemeNonul<CAP> {}
    impl<const CAP: usize> Grapheme for crate::GraphemeU8<CAP> {}
}

impl Grapheme for char {}
impl Grapheme for char7 {}
impl Grapheme for char8 {}
impl Grapheme for char16 {}
impl Grapheme for char_utf8 {}

#[cfg(feature = "alloc")]
impl Grapheme for crate::GraphemeString {}
