// devela_base_core::text::fmt::num::conf
//
//! Defines [`FmtNumGroup`].
//

use crate::{Boundary1d, Cmp, ConstInitCore, is};

#[doc = crate::_tags!(fmt num)]
/// Numeric digit grouping configuration.
#[doc = crate::_doc_location!("text/fmt")]
///
/// This type controls structural digit grouping for formatted numbers,
/// such as thousands separators, without performing layout or localization.
///
/// Grouping is applied independently to the integral (left) and fractional
/// (right) digit sequences, counting from the radix point outward.
///
/// A grouping length of `0` disables grouping on that side,
/// as well as having a `None` separator for that side.
///
/// This configuration is numeric-decoration only:
/// it does not handle spacing, alignment, text direction, or locale rules.
///
/// Grouping assumes 1-byte separators and ASCII digits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNumGroup {
    /// Number of digits per group on the integral (left) side.
    pub left_len: u8,
    /// Number of digits per group on the fractional (right) side.
    pub right_len: u8,
    /// Separator glyph inserted between digit groups on the integral (left) side.
    pub left_sep: Option<char>,
    /// Separator glyph inserted between digit groups on the fractional (right) side.
    pub right_sep: Option<char>,
}
#[rustfmt::skip]
impl Default for FmtNumGroup { fn default() -> Self { Self::INIT } }
impl ConstInitCore for FmtNumGroup {
    const INIT: Self = Self {
        left_len: 3,
        right_len: 0,
        left_sep: None,
        right_sep: None,
    };
}

#[rustfmt::skip]
impl FmtNumGroup {
    const _SIZE: () = const { assert![size_of::<Self>() == 12]; };

    /* constructors */

    /// Creates a default grouping configuration of 3 for the left part, and no separator.
    #[inline(always)]
    pub const fn new() -> Self { Self::INIT }

    /// Sets the `_` underscore for the left part, with groups of 3.
    pub const fn rust() -> Self { Self::INIT.with_sep('_') }
    ///
    /// Sets the `_` underscore for both parts, with groups of 3.
    pub const fn both_rust() -> Self { Self::INIT.with_sep('_').with_len(3) }

    /// Sets the `_` underscore for the left part, with groups of 4.
    pub const fn rust_hex() -> Self { Self::INIT.with_left(4, '_') }

    /// Sets the `_` underscore for both parts, with groups of 4.
    pub const fn both_rust_hex() -> Self { Self::INIT.with_sep('_').with_len(4) }

    /* individual lengths */

    /// Returns a copy with the left length set to the given value.
    pub const fn with_left_len(mut self, len: u8) -> Self { self.left_len = len; self }

    /// Returns a copy with the right length set to the given value.
    pub const fn with_right_len(mut self, len: u8) -> Self { self.right_len = len; self }

    /* individual separators */

    /// Returns a copy with the left separator set to the given character.
    pub const fn with_left_sep(mut self, sep: char) -> Self { self.left_sep = Some(sep); self }
    /// Return a copy with the left separator cleared.
    pub const fn without_left_sep(mut self) -> Self { self.left_sep = None; self }

    /// Returns a copy with the right separator set to the given character.
    pub const fn with_right_sep(mut self, sep: char) -> Self { self.right_sep = Some(sep); self }
    /// Return a copy with the right separator cleared.
    pub const fn without_right_sep(mut self) -> Self { self.right_sep = None; self }

    /* grouped */

    /// Returns a copy with both lenghts set to different values.
    pub const fn with_len(mut self, len: u8) -> Self {
        self.left_len = len; self.right_len = len; self }

    /// Returns a copy with both separators set to the same character.
    pub const fn with_sep(mut self, sep: char) -> Self {
        self.left_sep = Some(sep); self.right_sep = Some(sep); self }
    /// Returns a copy with both separators cleared.
    pub const fn without_sep(mut self) -> Self {
        self.left_sep = None; self.right_sep = None; self }

    /* sides */

    /// Returns whether left-side grouping is enabled.
    pub const fn has_left(&self) -> bool { self.left_len != 0 && self.left_sep.is_some() }

    /// Returns whether right-side grouping is enabled.
    pub const fn has_right(&self) -> bool { self.right_len != 0 && self.right_sep.is_some() }

    /// Returns whether any grouping is enabled.
    pub const fn is_enabled(&self) -> bool { self.has_left() || self.has_right() }

    /// Returns a copy with the left length and separator set to the given values.
    pub const fn with_left(mut self, len: u8, sep: char) -> Self {
        self.left_len = len; self.left_sep = Some(sep); self }

    /// Returns a copy with the right length and separator set to the given values.
    pub const fn with_right(mut self, len: u8, sep: char) -> Self {
        self.right_len = len; self.right_sep = Some(sep); self }
}

impl FmtNumGroup {
    /// Returns the grouping length for the given boundary side.
    ///
    /// For example, on the lower (left) side this corresponds to the
    /// number of digits per group in the integral part.
    pub const fn side_len(&self, side: Boundary1d) -> u8 {
        match side {
            Boundary1d::Lower => self.left_len,
            Boundary1d::Upper => self.right_len,
        }
    }

    /// Returns the grouping separator for the given boundary side, if any.
    pub const fn side_sep(&self, side: Boundary1d) -> Option<char> {
        match side {
            Boundary1d::Lower => self.left_sep,
            Boundary1d::Upper => self.right_sep,
        }
    }

    /// Returns whether digit grouping is enabled on the given boundary side.
    ///
    /// Grouping is enabled when both a non-zero grouping length and
    /// a separator are configured for that side.
    pub const fn side_enabled(&self, side: Boundary1d) -> bool {
        self.side_len(side) != 0 && self.side_sep(side).is_some()
    }

    /// Computes the minimal number of digits required so that, after grouping,
    /// the rendered width on the given boundary side is at least `min_width`.
    ///
    /// This accounts only for digit grouping on the specified side and does not
    /// include any sign or radix separator.
    pub const fn digits_for_grouped_width(
        &self,
        side: Boundary1d,
        digit_count: u16,
        min_width: u16,
    ) -> u16 {
        is![!self.side_enabled(side); return Cmp(digit_count).max(min_width)];
        let len = self.side_len(side) as u16;
        let mut digits = Cmp((min_width * len) / (len + 1)).max(digit_count);
        let seps = (digits.saturating_sub(1)) / len;
        is![digits + seps < min_width; digits += 1];
        digits
    }
}
