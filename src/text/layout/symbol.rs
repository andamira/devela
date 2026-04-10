// devela::text::layout::symbol
//
//! Defines [`TextCohesion`], [`TextLayoutSpan`], [`TextSymbol`].
//!
//! > Everything that describes what is being laid out.
//

use crate::{_impl_init, Interval, TextIndex, TextRange, TextUnit};

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
_impl_init![ConstInit: Self::Atomic => TextCohesion];

#[doc = crate::_tags!(text layout quant)]
/// Mapping between a contiguous text range and its consumed inline space.
#[doc = crate::_doc_location!("text/layout")]
///
/// A `TextLayoutSpan` records that a contiguous range of symbols contributed
/// a given amount of inline space during a layout step.
///
/// It is the primary structural output of text layout
/// and enables stable mapping between text indices and spatial occupation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextLayoutSpan {
    /// Range between the first and last symbols included in this span.
    pub range: TextRange,
    /// Total inline space consumed by the symbols in this span.
    pub units: TextUnit,
}
_impl_init![ConstInit: Self::from_prim(0, 0, 0) => TextLayoutSpan];

impl TextLayoutSpan {
    /// Creates a span from symbol indices and consumed units.
    pub const fn new(start: TextIndex, end: TextIndex, units: TextUnit) -> Self {
        Self { range: TextRange { start, end }, units }
    }
    /// Creates a span from symbol indices and consumed units.
    pub const fn with_range(range: TextRange, units: TextUnit) -> Self {
        Self { range, units }
    }

    /// Creates a span from symbol indices and consumed units.
    pub const fn from_prim(start: TextUnit, end: TextUnit, units: TextUnit) -> Self {
        Self::new(TextIndex(start), TextIndex(end), units)
    }

    /// Returns the start index.
    pub const fn start(self) -> TextIndex {
        self.range.start
    }

    /// Returns the end index.
    pub const fn end(self) -> TextIndex {
        self.range.end
    }

    /// Returns the interval from `start` to `end`.
    pub const fn interval(self) -> Interval<TextIndex> {
        Interval::closed_open(self.start(), self.end())
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
_impl_init![ConstInit: Self::new(TextUnit::INIT, TextCohesion::INIT) => TextSymbol];

impl TextSymbol {
    /// Creates a span from symbol indices and consumed units.
    pub const fn new(units: TextUnit, cohesion: TextCohesion) -> Self {
        Self { units, cohesion }
    }
}
