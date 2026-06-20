// devela/src/text/layout/cell.rs
//
//! Defines [`TextCellWidthMode`].
//

use crate::_impl_init;

#[doc = crate::_tags!(text layout)]
/// Inline width policy for derived layout symbols.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextCellWidthMode = 1|8),
}]
///
/// This controls how much abstract inline space a derived symbol consumes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextCellWidthMode {
    /// Every derived symbol consumes one unit.
    #[default]
    Mono,

    /// Use East Asian width rules where applicable.
    EastAsian,
}
_impl_init![Self::Mono => TextCellWidthMode];
