// devela_base_core::text::unit
//
//! Defines [`TextUnit`], [`TextIndex`], [`TextCursor`], [`TextRange`].
//!
//! > Low-level scalar and addressing types.
//

use crate::_impl_init;

#[doc = crate::_tags!(text)]
/// Primitive text-domain unit.
#[doc = crate::_doc_location!("text")]
///
/// `TextUnit` is the base primitive quantity used by text-related abstractions.
///
/// Depending on context, it may represent:
/// - positional magnitude in a text sequence
/// - inline extent or consumed space during layout
/// - other caller-defined text quantities
///
/// It has no inherent physical meaning and does not correspond directly to pixels,
/// glyphs, or characters unless a downstream layer defines that interpretation.
///
/// More specific wrappers such as [`TextIndex`] refine this primitive into a stricter role.
pub type TextUnit = u32;

#[doc = crate::_tags!(text layout)]
/// Position within a caller-defined text-oriented sequence.
#[doc = crate::_doc_location!("text")]
///
/// The sequence may represent bytes, Unicode scalars, graphemes, layout symbols,
/// or other text-derived units chosen by the caller.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextIndex(pub u32);
_impl_init![ConstInitCore: Self(0) => TextIndex];

// pub struct TextOffset(pub i32); // MAYBE

#[rustfmt::skip]
impl TextIndex {
    /// Creates a new text index.
    pub const fn new(value: u32) -> Self { Self(value) }

    /// Returns the next index.
    /// # Panics
    /// Panics on overflow.
    pub const fn next(self) -> Self { Self(self.0 + 1) }

    /// Returns the next index, or `None` on overflow.
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Converts to `usize` for indexing slices.
    pub const fn as_usize(self) -> usize { self.0 as usize }
}

#[doc = crate::_tags!(text)]
/// Continuation point within a caller-defined text traversal.
#[doc = crate::_doc_location!("text")]
///
/// A `TextCursor` marks where a text-oriented process should resume.
///
/// The underlying sequence may represent bytes, Unicode scalars, graphemes,
/// layout symbols, or other caller-defined text units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextCursor {
    /// Index of the next element in the traversal sequence.
    pub index: TextIndex,
}
_impl_init![ConstInitCore: Self::new(0) => TextCursor];

impl TextCursor {
    /// Creates a new cursor at the given index.
    pub const fn new(index: u32) -> Self {
        Self { index: TextIndex(index) }
    }
}

#[doc = crate::_tags!(text)]
/// A half-open range within a caller-defined text-oriented sequence.
#[doc = crate::_doc_location!("text")]
///
/// `TextRange` covers the interval `[start, end)`,
/// including `start` and excluding `end`.
///
/// The underlying sequence may represent bytes, Unicode scalars, graphemes,
/// layout symbols, or other caller-defined text units.
///
/// Empty ranges are valid and satisfy `start == end`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextRange {
    /// The first index included in the range.
    pub start: TextIndex,
    /// The first index excluded from the range.
    pub end: TextIndex,
}
impl TextRange {
    /// Creates a new half-open range from text indices.
    /// # Panics
    /// Panics if `start > end`.
    pub const fn new(start: TextIndex, end: TextIndex) -> Self {
        assert!(start.0 <= end.0, "TextRange requires start <= end");
        Self { start, end }
    }

    /// Creates a range from primitive indices.
    /// # Panics
    /// Panics if `start > end`.
    pub const fn from_prim(start: TextUnit, end: TextUnit) -> Self {
        Self::new(TextIndex(start), TextIndex(end))
    }

    /// Creates some range from a start index and a length, or `None` on overflow.
    pub const fn from_start_len(start: TextIndex, len: TextUnit) -> Option<Self> {
        match start.0.checked_add(len) {
            Some(end) => Some(Self { start, end: TextIndex(end) }),
            None => None,
        }
    }

    /// Returns `true` if the range is empty.
    pub const fn is_empty(self) -> bool {
        self.start.0 == self.end.0
    }
    /// Returns the range length in sequence units.
    pub const fn len(self) -> TextUnit {
        self.end.0 - self.start.0
    }
}
