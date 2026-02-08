// devela_base_core::text::layout::symbol
//
//! Defines [`TextCohesion`], [`TextCursor`], [`TextSpan`], [`TextSymbol`].
//!
//! > Everything that describes what is being laid out.
//

use crate::{_impl_init, Interval, TextIndex, TextUnit};

#[doc = crate::_tags!(text layout)]
/// Spatial cohesion rules for a text symbol during layout.
#[doc = crate::_doc_location!("text/layout")]
///
/// This enum defines what a layout engine is allowed to do with a symbol
/// when negotiating limited inline space. It expresses layout permissions only;
/// it does not encode meaning, style, or language semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextCohesion {
    /// Must be consumed as a whole or not at all.
    ///
    /// The symbol may not be split or removed.
    /// If it does not fit, layout must stop or fail the fit decision.
    Atomic,

    /// May be split across layout steps.
    ///
    /// The symbol provides a soft break opportunity
    /// and may be partially consumed if space runs out.
    Breakable,

    /// May be omitted to satisfy space constraints.
    ///
    /// The symbol may be entirely removed from the layout result
    /// if needed to achieve a fit.
    Elidable,
}
_impl_init![ConstInitCore: Self::Atomic => TextCohesion];

#[doc = crate::_tags!(text layout)]
/// Continuation point in a text symbol stream.
#[doc = crate::_doc_location!("text/layout")]
///
/// A `TextCursor` identifies where layout should resume when text
/// is processed incrementally across multiple layout steps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] // MAYBE Default
pub struct TextCursor {
    /// Index of the next symbol to be considered for layout.
    pub index: TextIndex,
}
_impl_init![ConstInitCore: Self::new(0) => TextCursor];

impl TextCursor {
    /// Creates a new cursor at the given symbol index.
    pub const fn new(index: u32) -> Self {
        Self { index: TextIndex(index) }
    }
}

#[doc = crate::_tags!(text layout quant)]
/// Mapping between a contiguous text range and its consumed inline space.
#[doc = crate::_doc_location!("text/layout")]
///
/// A `TextSpan` records that a contiguous range of symbols contributed
/// a given amount of inline space during a layout step.
///
/// It is the primary structural output of text layout
/// and enables stable mapping between text indices and spatial occupation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextSpan {
    /// Index of the first symbol included in this span.
    pub start: TextIndex,
    /// Index immediately after the last symbol included in this span.
    pub end: TextIndex,
    /// Total inline space consumed by the symbols in this span.
    pub units: TextUnit,
}
_impl_init![ConstInitCore: Self::from_prim(0, 0, 0) => TextSpan];

impl TextSpan {
    /// Creates a span from symbol indices and consumed units.
    pub const fn new(start: TextIndex, end: TextIndex, units: TextUnit) -> Self {
        Self { start, end, units }
    }
    /// Creates a span from symbol indices and consumed units.
    pub const fn from_prim(start: u32, end: u32, units: TextUnit) -> Self {
        Self {
            start: TextIndex(start),
            end: TextIndex(end),
            units,
        }
    }

    /// Returns the interval from `start` to `end`.
    pub const fn interval(self) -> Interval<TextIndex> {
        Interval::closed_open(self.start, self.end)
    }
}

#[doc = crate::_tags!(text layout io)]
/// Layout participation record for a single text symbol.
#[doc = crate::_doc_location!("text/layout")]
///
/// A `TextSymbol` describes how a unit of text participates in layout:
/// how much inline space it consumes and what spatial constraints apply under limited space.
///
/// It carries no identity or semantic meaning beyond layout behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextSymbol {
    /// Inline space consumed by this symbol when laid out as a whole.
    pub units: TextUnit,
    /// Spatial cohesion rules applied to this symbol under limited space.
    pub cohesion: TextCohesion,
}
_impl_init![ConstInitCore: Self::new(TextUnit::INIT, TextCohesion::INIT) => TextSymbol];

impl TextSymbol {
    /// Creates a span from symbol indices and consumed units.
    pub const fn new(units: TextUnit, cohesion: TextCohesion) -> Self {
        Self { units, cohesion }
    }
}
