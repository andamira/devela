// devela/src/text/layout/config.rs
//
//! Defines text layout configuration policies.
//

use crate::_impl_init;

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
/// Inline width policy for derived layout symbols.
#[doc = crate::_doc_meta!{location("text/layout")}]
///
/// This controls how much abstract inline space a derived symbol consumes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextWidthMode {
    /// Every derived symbol consumes one unit.
    #[default]
    Mono,

    /// Use East Asian width rules where applicable.
    ///
    /// This remains abstract and does not imply pixels, fonts, terminals, or DPI.
    EastAsian,
}
_impl_init![Self::Mono => TextWidthMode];

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
/// Policy bundle for deriving layout symbols from text.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextSymbolConfig = 3 | 24),
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
    pub width_mode: TextWidthMode,

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
        width_mode: TextWidthMode::Mono,
        elide_mode: TextElideMode::None,
    };
    /// Creates a new text symbol configuration.
    #[must_use]
    pub const fn new(
        break_mode: TextBreakMode,
        width_mode: TextWidthMode,
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
        Self::new(TextBreakMode::Grapheme, TextWidthMode::Mono, TextElideMode::None)
    }
    /// Creates a word-based monospace configuration.
    #[must_use]
    pub const fn word_mono() -> Self {
        Self::new(TextBreakMode::Word, TextWidthMode::Mono, TextElideMode::None)
    }
    /// Returns this configuration with a different break mode.
    #[must_use]
    pub const fn with_break_mode(mut self, break_mode: TextBreakMode) -> Self {
        self.break_mode = break_mode;
        self
    }
    /// Returns this configuration with a different width mode.
    #[must_use]
    pub const fn with_width_mode(mut self, width_mode: TextWidthMode) -> Self {
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
