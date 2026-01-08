// devela_base_core::text::fmt::num::shape
//
//! Defines [`FmtNumShape`].
//

use crate::is;

#[doc = crate::_tags!(fmt num)]
/// Describes the structural shape of a formatted number.
#[doc = crate::_doc_location!("text/fmt")]
///
/// The shape captures the lengths of the numeric regions and prefixes,
/// independent of padding, alignment, or styling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNumShape {
    /// Number of prefix glyphs (e.g. sign or base prefix).
    pub prefix: u16,

    /// Number of digits in the integral (left) part.
    pub left: u16,

    /// Number of digits in the fractional (right) part.
    pub right: u16,
}

impl FmtNumShape {
    /// Creates a numeric shape from prefix and digit counts.
    ///
    /// - `prefix` counts leading non-digit glyphs (e.g. sign or base prefix).
    /// - `left` counts digits in the integral (left) part.
    /// - `right` counts digits in the fractional (right) part.
    #[inline(always)]
    pub const fn new(prefix: u16, left: u16, right: u16) -> Self {
        Self { prefix, left, right }
    }

    /// Returns whether the number has a radix separator.
    ///
    /// This is true when there is a fractional part (`right > 0`).
    #[inline(always)]
    pub const fn has_radix(&self) -> bool {
        self.right > 0
    }

    /// Returns the total number of glyphs required to render the number.
    ///
    /// This includes:
    /// - the prefix (e.g. sign),
    /// - integral digits,
    /// - the radix separator (if present),
    /// - fractional digits.
    #[inline(always)]
    pub const fn total(&self) -> usize {
        self.prefix as usize
            + self.left as usize
            + is![self.has_radix(); 1; 0]
            + self.right as usize
    }

    /// Returns the size of the left alignment block.
    ///
    /// This includes the prefix and all integral digits, but excludes
    /// the radix separator and fractional digits.
    #[inline(always)]
    pub const fn left_block(self) -> usize {
        (self.prefix + self.left) as usize
    }
}
