// devela_base_core::text::layout::unit
//
//! Defines [`TextUnit`], [`TextIndex`].
//!
//! > Low-level scalar and addressing types.
//
// TODO: support niche optimization and custom handle.

use crate::_impl_init;

#[doc = crate::_tags!(text layout)]
/// Abstract inline layout unit.
#[doc = crate::_doc_location!("text/layout")]
///
/// `TextUnit` represents how much inline space text consumes during layout.
///
/// It has no inherent physical meaning and does not correspond to pixels, glyphs, or characters.
///
/// Interpretation of units (cells, ems, points…) is delegated to downstream realization layers.
pub type TextUnit = u32;

#[doc = crate::_tags!(text layout)]
/// Address into a text symbol stream.
#[doc = crate::_doc_location!("text/layout")]
///
/// `TextIndex` identifies a position within the caller-provided sequence of text symbols.
///
/// It does not imply any particular text representation (bytes, characters, graphemes, glyphs).
///
/// The meaning of a symbol is defined by the producer of the symbol stream,
/// not by the layout engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextIndex(pub u32);
_impl_init![ConstInitCore: Self(0) => TextIndex];

#[rustfmt::skip]
impl TextIndex {
    /// Creates a new text index.
    pub const fn new(value: u32) -> Self { Self(value) }

    /// Advances the index by one symbol.
    pub const fn next(self) -> Self { Self(self.0 + 1) }

    /// Converts to `usize` for indexing slices.
    pub const fn as_usize(self) -> usize { self.0 as usize }
}
