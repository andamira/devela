// devela/src/text/layout/symbol.rs
//
//! Defines [`TextBreakMode`], [`TextElideMode`], [`TextCohesion`],
//! [`TextLayoutSpan`], [`TextSymbol`], [`TextSymbolConfig`].
//!
//! > Everything about symbols and how they are produced.
//

use crate::{_impl_init, Interval, TextIndex, TextRange, TextUnit, TextelWidthMode};

#[doc = crate::_tags!(text layout)]
/// Boundary policy for deriving layout symbols from text.
#[doc = crate::_doc_meta!{location("text/layout")}]
///
/// This controls where text may become separate layout symbols before
/// inline layout negotiation happens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextBreakMode {
    /// Break only at explicit whitespace boundaries.
    #[default]
    Whitespace,

    /// Break at word-like boundaries.
    ///
    /// This is intended for Unicode-aware word segmentation when available.
    Word,

    /// Break between grapheme clusters.
    ///
    /// This is useful for CJK-friendly wrapping and fine-grained cursor mapping.
    Grapheme,
}
_impl_init![Self::Whitespace => TextBreakMode];

#[doc = crate::_tags!(text layout)]
/// Elision policy for derived layout symbols.
#[doc = crate::_doc_meta!{location("text/layout")}]
///
/// This controls which derived symbols may disappear during layout negotiation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextElideMode {
    /// No derived symbol is marked as elidable.
    #[default]
    None,

    /// Whitespace-like separators may be marked as elidable.
    Whitespace,
}
_impl_init![Self::None => TextElideMode];

#[doc = crate::_tags!(text layout)]
/// Spatial cohesion rules for a text symbol during layout.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextCohesion = 1|8),
}]
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
_impl_init![Self::Atomic => TextCohesion];

#[doc = crate::_tags!(text layout quant)]
/// Mapping between a contiguous text range and its consumed inline space.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextLayoutSpan = 12|96),
}]
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
_impl_init![Self::from_prim(0, 0, 0) => TextLayoutSpan];

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
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextSymbol = 8|64),
}]
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
_impl_init![Self::new(TextUnit::INIT, TextCohesion::INIT) => TextSymbol];

impl TextSymbol {
    /// Creates a span from symbol indices and consumed units.
    pub const fn new(units: TextUnit, cohesion: TextCohesion) -> Self {
        Self { units, cohesion }
    }
}

#[doc = crate::_tags!(text layout)]
/// Policy bundle for deriving layout symbols from text.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextSymbolConfig = 3|24),
}]
///
/// `TextSymbolConfig` describes how text is converted into layout symbols:
/// where boundaries may occur, how widths are measured, and which symbols
/// may be omitted during layout negotiation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextSymbolConfig {
    /// Boundary policy for derived symbols.
    pub break_mode: TextBreakMode,

    /// Inline width policy for derived symbols.
    pub width_mode: TextelWidthMode,

    /// Elision policy for derived symbols.
    pub elide_mode: TextElideMode,
}
_impl_init![Self::DEFAULT => TextSymbolConfig];
impl Default for TextSymbolConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl TextSymbolConfig {
    /// Default text symbol configuration.
    pub const DEFAULT: Self = Self {
        break_mode: TextBreakMode::Whitespace,
        width_mode: TextelWidthMode::Mono,
        elide_mode: TextElideMode::None,
    };
    /// Creates a new text symbol configuration.
    #[must_use]
    pub const fn new(
        break_mode: TextBreakMode,
        width_mode: TextelWidthMode,
        elide_mode: TextElideMode,
    ) -> Self {
        Self { break_mode, width_mode, elide_mode }
    }

    /// Creates a whitespace-based monospace configuration.
    #[must_use]
    pub const fn whitespace_mono() -> Self {
        Self::DEFAULT
    }
    /// Creates a grapheme-based monospace configuration.
    #[must_use]
    pub const fn grapheme_mono() -> Self {
        Self::new(TextBreakMode::Grapheme, TextelWidthMode::Mono, TextElideMode::None)
    }
    /// Creates a word-based monospace configuration.
    #[must_use]
    pub const fn word_mono() -> Self {
        Self::new(TextBreakMode::Word, TextelWidthMode::Mono, TextElideMode::None)
    }
    /// Returns this configuration with a different break mode.
    #[must_use]
    pub const fn with_break_mode(mut self, break_mode: TextBreakMode) -> Self {
        self.break_mode = break_mode;
        self
    }
    /// Returns this configuration with a different width mode.
    #[must_use]
    pub const fn with_width_mode(mut self, width_mode: TextelWidthMode) -> Self {
        self.width_mode = width_mode;
        self
    }
    /// Returns this configuration with a different elide mode.
    #[must_use]
    pub const fn with_elide_mode(mut self, elide_mode: TextElideMode) -> Self {
        self.elide_mode = elide_mode;
        self
    }
}
