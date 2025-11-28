// devela_base_text::text::grapheme::scanner::properties

use super::trie::graphemes_lookup;
use crate::{Mem, charu, impl_trait};

#[doc = crate::_TAG_TEXT!()]
#[doc = concat![crate::_ABBR_EGC!(), "property values from Unicode Standard Annex #29."]]
///
/// Used by the grapheme boundary detection algorithm to determine where
/// grapheme cluster boundaries occur in text.
///
/// Based on the **Grapheme_Cluster_Break** property from [UAX#29 Section 3.1][0].
///
/// Note: `ExtendedPictographic` is derived from emoji code point tables
/// but treated as mutually exclusive with other break properties.
///
/// [0]: https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Break_Property_Values
///
#[doc = crate::_doc!(vendor: "grapheme_machine")]
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Eq)]
pub enum GraphemePropCb {
    /// No specific grapheme cluster break property applies
    None = 0x00,
    /// Carriage Return (U+000D)
    CR = 0x01,
    /// Control code point (general category Cc, Cf, Zl, or Zp)
    Control = 0x02,
    /// Grapheme extender (general category Me or Mn)
    Extend = 0x03,
    /// Extended pictographic code point (emoji)
    ExtendedPictographic = 0x04,
    /// Hangul syllable L (leading consonant)
    L = 0x05,
    /// Line Feed (U+000A)
    LF = 0x06,
    /// Hangul syllable LV (vowel)
    LV = 0x07,
    /// Hangul syllable LVT (trailing consonant)
    LVT = 0x08,
    /// Prepend code point
    Prepend = 0x09,
    /// Regional indicator symbol (flag emojis)
    RegionalIndicator = 0x0a,
    /// Spacing mark
    SpacingMark = 0x0b,
    /// Hangul syllable T (trailing consonant)
    T = 0x0c,
    /// Hangul syllable V (vowel)
    V = 0x0d,
    /// Zero Width Joiner (U+200D)
    Zwj = 0x0e,
}

impl_trait! { PartialEq for GraphemePropCb |self, other| Self::eq(*self, *other) }
impl_trait! { Hash for GraphemePropCb |self, state| { Mem::discriminant(self).hash(state); } }

impl GraphemePropCb {
    /// Const-compatible `Eq`.
    pub const fn eq(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::None, Self::None)
                | (Self::CR, Self::CR)
                | (Self::Control, Self::Control)
                | (Self::Extend, Self::Extend)
                | (Self::ExtendedPictographic, Self::ExtendedPictographic)
                | (Self::L, Self::L)
                | (Self::LF, Self::LF)
                | (Self::LV, Self::LV)
                | (Self::LVT, Self::LVT)
                | (Self::Prepend, Self::Prepend)
                | (Self::RegionalIndicator, Self::RegionalIndicator)
                | (Self::SpacingMark, Self::SpacingMark)
                | (Self::T, Self::T)
                | (Self::V, Self::V)
                | (Self::Zwj, Self::Zwj)
        )
    }
}

#[doc = crate::_TAG_TEXT!()]
/// Break property for Indic scripts that prevents splitting within orthographic syllables.
///
/// Used by grapheme boundary rule [GB9c](https://www.unicode.org/reports/tr29/#GB9c)
/// to avoid inappropriate breaks in conjunct sequences.
///
/// Based on the **Indic_Conjunct_Break** property from Unicode.
///
#[doc = crate::_doc!(vendor: "grapheme_machine")]
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Eq)]
pub enum GraphemePropInCb {
    /// Code point is not part of an Indic conjunct sequence.
    None = 0x00,
    /// An Indic consonant code point.
    Consonant = 0x10,
    /// An extending code point in an Indic sequence.
    Extend = 0x20,
    /// A linker code point in an Indic sequence.
    Linker = 0x30,
}

impl_trait! { PartialEq for GraphemePropInCb |self, other| Self::eq(*self, *other) }
impl_trait! { Hash for GraphemePropInCb |self, state| { Mem::discriminant(self).hash(state); } }

impl GraphemePropInCb {
    /// Const-compatible `Eq`.
    pub const fn eq(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::None, Self::None)
                | (Self::Consonant, Self::Consonant)
                | (Self::Extend, Self::Extend)
                | (Self::Linker, Self::Linker)
        )
    }
}

#[doc = crate::_TAG_TEXT!()]
#[doc = concat!["Combined ", crate::_ABBR_EGC!(), "break properties for a single code point."]]
///
/// Packed representation of both [`GraphemePropCb`] and [`GraphemePropInCb`]
/// properties used by Unicode grapheme cluster [boundary rules].
///
#[doc = crate::_doc!(vendor: "grapheme_machine")]
///
/// [boundary rules]: https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundary_Rules
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq)]
pub struct GraphemeProps {
    ///  Stores `GraphemePropCb` in the low 4 bits and `GraphemePropInCb` in bits 4-5,
    ///  allowing efficient property checks.
    raw: u8,
}

impl_trait! { PartialEq for GraphemeProps |self, other| Self::eq(*self, *other) }
impl_trait! { Hash for GraphemeProps |self, state| { self.raw.hash(state); } }

impl GraphemeProps {
    /// Creates a new property tuple from individual break properties.
    pub const fn new(gcb: GraphemePropCb, incb: GraphemePropInCb) -> Self {
        Self { raw: gcb as u8 | incb as u8 }
    }

    /// Looks up grapheme properties for a scalar.
    ///
    /// Uses an embedded trie to find the Grapheme_Cluster_Break and
    /// Indic_Conjunct_Break properties for the scalar.
    pub const fn for_charu(c: charu) -> Self {
        Self { raw: graphemes_lookup(c) }
    }

    /// Looks up grapheme properties for a scalar.
    ///
    /// Convenience method that converts to `charu` internally.
    /// Prefer `for_charu` if you already have a `charu`.
    pub const fn for_char(c: char) -> Self {
        Self { raw: graphemes_lookup(charu::from_char(c)) }
    }

    /// Returns the Grapheme_Cluster_Break property.
    ///
    /// # Features
    /// Uses the `unsafe_layout` feature to transmute instead of match.
    pub const fn gcb_property(self) -> GraphemePropCb {
        #[cfg(any(base_safe_text, not(feature = "unsafe_layout")))]
        match self.raw & 0xf {
            0x00 => GraphemePropCb::None,
            0x01 => GraphemePropCb::CR,
            0x02 => GraphemePropCb::Control,
            0x03 => GraphemePropCb::Extend,
            0x04 => GraphemePropCb::ExtendedPictographic,
            0x05 => GraphemePropCb::L,
            0x06 => GraphemePropCb::LF,
            0x07 => GraphemePropCb::LV,
            0x08 => GraphemePropCb::LVT,
            0x09 => GraphemePropCb::Prepend,
            0x0a => GraphemePropCb::RegionalIndicator,
            0x0b => GraphemePropCb::SpacingMark,
            0x0c => GraphemePropCb::T,
            0x0d => GraphemePropCb::V,
            0x0e => GraphemePropCb::Zwj,
            _ => unreachable!(), // mask ensures only 0x00-0x0E
        }
        #[cfg(all(not(base_safe_text), feature = "unsafe_layout"))]
        {
            let raw = self.raw & 0xf;
            // SAFETY: The low nibble of raw repr matches the GraphemePropCb repr.
            unsafe { core::mem::transmute(raw) }
        }
    }

    /// Returns the Indic_Conjunct_Break property.
    ///
    /// # Features
    /// Uses the `unsafe_layout` feature to transmute instead of match.
    pub const fn incb_property(self) -> GraphemePropInCb {
        #[cfg(any(base_safe_text, not(feature = "unsafe_layout")))]
        match self.raw & 0x30 {
            0x00 => GraphemePropInCb::None,
            0x10 => GraphemePropInCb::Consonant,
            0x20 => GraphemePropInCb::Extend,
            0x30 => GraphemePropInCb::Linker,
            _ => unreachable!(), // mask ensures only these values
        }
        #[cfg(all(not(base_safe_text), feature = "unsafe_layout"))]
        {
            let raw = self.raw & 0x30;
            // SAFETY: The selected bits of raw repr matches ththe GraphemePropInCb repr.
            unsafe { core::mem::transmute(raw) }
        }
    }

    /// Returns `true` if this code point is a control code point.
    ///
    /// Matches CR, LF, or Control categories for rules [GB4]/[GB5].
    ///
    /// [GB4]: https://www.unicode.org/reports/tr29/#GB4
    /// [GB5]: https://www.unicode.org/reports/tr29/#GB5
    pub const fn is_any_control(self) -> bool {
        matches!(
            self.gcb_property(),
            GraphemePropCb::LF | GraphemePropCb::CR | GraphemePropCb::Control,
        )
    }

    /// Const-compatible `Eq`.
    pub const fn eq(self, other: Self) -> bool {
        self.raw == other.raw
    }
}

#[cfg(test)]
#[allow(unused, non_upper_case_globals)]
impl GraphemeProps {
    pub(crate) const None: Self = Self::gcb_only(GraphemePropCb::None);
    pub(crate) const CR: Self = Self::gcb_only(GraphemePropCb::CR);
    pub(crate) const Control: Self = Self::gcb_only(GraphemePropCb::Control);
    pub(crate) const Extend: Self = Self::gcb_only(GraphemePropCb::Extend);
    pub(crate) const ExtendedPictographic: Self =
        Self::gcb_only(GraphemePropCb::ExtendedPictographic);
    pub(crate) const L: Self = Self::gcb_only(GraphemePropCb::L);
    pub(crate) const LF: Self = Self::gcb_only(GraphemePropCb::LF);
    pub(crate) const LV: Self = Self::gcb_only(GraphemePropCb::LV);
    pub(crate) const LVT: Self = Self::gcb_only(GraphemePropCb::LVT);
    pub(crate) const Prepend: Self = Self::gcb_only(GraphemePropCb::Prepend);
    pub(crate) const RegionalIndicator: Self = Self::gcb_only(GraphemePropCb::RegionalIndicator);
    pub(crate) const SpacingMark: Self = Self::gcb_only(GraphemePropCb::SpacingMark);
    pub(crate) const T: Self = Self::gcb_only(GraphemePropCb::T);
    pub(crate) const V: Self = Self::gcb_only(GraphemePropCb::V);
    pub(crate) const Zwj: Self = Self::gcb_only(GraphemePropCb::Zwj);

    const fn gcb_only(gcb: GraphemePropCb) -> Self {
        Self::new(gcb, GraphemePropInCb::None)
    }
}
