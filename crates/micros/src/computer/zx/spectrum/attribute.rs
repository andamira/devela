//
//! Defines [`SpectrumAttribute`].
//

use crate::SpectrumColor;

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
    /// Attribute mask for flashing ink and paper.
    pub const FLASH_MASK: u8 = 0x80;

    /// Attribute mask for bright colors.
    pub const BRIGHT_MASK: u8 = 0x40;

    /// Creates an attribute with the given ink and paper colors.
    #[must_use]
    pub const fn new(ink: SpectrumColor, paper: SpectrumColor) -> Self {
        Self((paper as u8) << 3 | ink as u8)
    }

    /// Returns the raw attribute byte.
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    /// Returns the ink color.
    #[must_use]
    pub const fn ink(self) -> SpectrumColor {
        SpectrumColor::from_u8(self.0 & 0x07)
    }
    /// Returns the paper color.
    #[must_use]
    pub const fn paper(self) -> SpectrumColor {
        SpectrumColor::from_u8((self.0 >> 3) & 0x07)
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
