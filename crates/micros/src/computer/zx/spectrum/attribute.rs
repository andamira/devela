//
//! Defines [`SpectrumAttribute`].
//

use crate::{SpectrumColor, is};

#[doc = crate::_tags!(hw color)]
/// One ZX Spectrum display attribute byte.
#[doc = crate::_doc_meta!{
    location("computer/zx", struct SpectrumAttribute),
    test_size_of(SpectrumAttribute = 1|8; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpectrumAttribute(u8);

impl SpectrumAttribute {
    /// Attribute mask for the ink colour.
    pub const INK_MASK: u8 = 0x07;
    /// Attribute mask for the paper colour.
    pub const PAPER_MASK: u8 = 0x38;
    /// Attribute mask for bright colours.
    pub const BRIGHT_MASK: u8 = 0x40;
    /// Attribute mask for flashing ink and paper.
    pub const FLASH_MASK: u8 = 0x80;

    /// Creates an attribute with the given ink and paper colours.
    #[must_use]
    pub const fn new(ink: SpectrumColor, paper: SpectrumColor) -> Self {
        Self((paper as u8) << 3 | ink as u8)
    }

    /// Creates an attribute from its raw byte representation.
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        Self(value)
    }
    /// Returns the raw attribute byte.
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    /// Returns the ink colour.
    #[must_use]
    pub const fn ink(self) -> SpectrumColor {
        SpectrumColor::from_u8(self.0 & Self::INK_MASK)
    }
    /// Returns this attribute with a different ink colour.
    #[must_use]
    pub const fn with_ink(self, color: SpectrumColor) -> Self {
        Self((self.0 & !Self::INK_MASK) | color as u8)
    }

    /// Returns the paper colour.
    #[must_use]
    pub const fn paper(self) -> SpectrumColor {
        SpectrumColor::from_u8((self.0 & Self::PAPER_MASK) >> 3)
    }
    /// Returns this attribute with a different paper colour.
    #[must_use]
    pub const fn with_paper(self, color: SpectrumColor) -> Self {
        Self((self.0 & !Self::PAPER_MASK) | (color as u8) << 3)
    }

    /// Returns this attribute with brightness enabled.
    #[must_use]
    pub const fn bright(self) -> Self {
        Self(self.0 | Self::BRIGHT_MASK)
    }
    /// Returns this attribute with flashing enabled.
    #[must_use]
    pub const fn flash(self) -> Self {
        Self(self.0 | Self::FLASH_MASK)
    }

    /// Returns this attribute with brightness changed.
    #[must_use]
    pub const fn with_bright(self, enabled: bool) -> Self {
        is! { enabled, Self(self.0 | Self::BRIGHT_MASK), Self(self.0 & !Self::BRIGHT_MASK) }
    }
    /// Returns this attribute with flashing changed.
    #[must_use]
    pub const fn with_flash(self, enabled: bool) -> Self {
        is! { enabled, Self(self.0 | Self::FLASH_MASK), Self(self.0 & !Self::FLASH_MASK) }
    }

    /// Returns whether brightness is enabled.
    #[must_use]
    pub const fn is_bright(self) -> bool {
        self.0 & Self::BRIGHT_MASK != 0
    }
    /// Returns whether flashing is enabled.
    #[must_use]
    pub const fn is_flash(self) -> bool {
        self.0 & Self::FLASH_MASK != 0
    }
}
