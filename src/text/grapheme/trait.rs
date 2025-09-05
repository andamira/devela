// devela::text::grapheme::trait
//
//!
//

use crate::GraphemeU8;

#[doc = crate::TAG_TEXT!()]
/// Common trait for <abbr title="Extended Grapheme Cluster">EGC</abbr> types.
pub trait Grapheme {}

impl<const CAP: usize> Grapheme for GraphemeU8<CAP> {}
