// devela::text::grapheme::trait
//
//!
//

use crate::{GraphemeNonul, GraphemeU8};

#[doc = crate::_TAG_TEXT!()]
/// Common trait for <abbr title="Extended Grapheme Cluster">EGC</abbr> types.
pub trait Grapheme {}

impl<const CAP: usize> Grapheme for GraphemeNonul<CAP> {}
impl<const CAP: usize> Grapheme for GraphemeU8<CAP> {}

#[cfg(feature = "alloc")]
impl Grapheme for crate::GraphemeString {}
