//
//! Defines [`SpectrumColor`].
//

#[allow(missing_docs)]
#[doc = crate::_tags!(hw color)]
/// The 3-bit base colors used by the Sinclair ZX Spectrum.
#[doc = crate::_doc_meta!{
    location("computer/zx", enum ComputerSpectrum48),
    test_size_of(SpectrumColor = 1|8; niche Option),
}]
/// These values encode ink, paper, and border colors.
/// Display brightness is controlled separately by the `BRIGHT` attribute bit.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpectrumColor {
    Black = 0,
    Blue = 1,
    Red = 2,
    Magenta = 3,
    Green = 4,
    Cyan = 5,
    Yellow = 6,
    White = 7,
}

impl SpectrumColor {
    /// Returns the Spectrum color encoded by the low 3 bits of `value`.
    ///
    /// Defaults to `White` for any `value` >=7.
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        match value & 0x07 {
            0 => Self::Black,
            1 => Self::Blue,
            2 => Self::Red,
            3 => Self::Magenta,
            4 => Self::Green,
            5 => Self::Cyan,
            6 => Self::Yellow,
            _ => Self::White,
        }
    }
}
