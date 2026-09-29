//
//! Defines [`SpectrumUlaOut`].
//

use crate::{SpectrumColor, is};

#[doc = crate::_tags!(hw io state)]
/// Output state written to the ZX Spectrum ULA port.
#[doc = crate::_doc_meta! {
    location("computer/zx", struct SpectrumUlaOut),
    test_size_of(SpectrumUlaOut = 1|8; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SpectrumUlaOut(u8);

impl SpectrumUlaOut {
    const BORDER_MASK: u8 = 0x07;
    const MIC_MASK: u8 = 0x08;
    const EAR_MASK: u8 = 0x10;

    /// Creates an output state with the given border colour.
    #[must_use]
    pub const fn new(border: SpectrumColor) -> Self {
        Self(border as u8)
    }
    /// Returns the raw ULA output byte.
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    /// Returns the selected border colour.
    #[must_use]
    pub const fn border(self) -> SpectrumColor {
        SpectrumColor::from_u8(self.0)
    }
    /// Returns this state with a different border colour.
    #[must_use]
    pub const fn with_border(self, color: SpectrumColor) -> Self {
        Self((self.0 & !Self::BORDER_MASK) | color as u8)
    }

    /// Returns whether the MIC output bit is set.
    #[must_use]
    pub const fn mic(self) -> bool {
        self.0 & Self::MIC_MASK != 0
    }
    /// Returns this state with the MIC output bit changed.
    #[must_use]
    pub const fn with_mic(self, enabled: bool) -> Self {
        is! { enabled, Self(self.0 | Self::MIC_MASK), Self(self.0 & !Self::MIC_MASK) }
    }

    /// Returns whether the EAR output bit is set.
    #[must_use]
    pub const fn ear(self) -> bool {
        self.0 & Self::EAR_MASK != 0
    }
    /// Returns this state with the EAR output bit changed.
    #[must_use]
    pub const fn with_ear(self, enabled: bool) -> Self {
        is! { enabled, Self(self.0 | Self::EAR_MASK), Self(self.0 & !Self::EAR_MASK) }
    }
}
