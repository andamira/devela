// devela_base_core::media::audio::layout
//
//! Defines [`AudioChannel`], [`AudioChannels`].
//
// https://en.wikipedia.org/wiki/Low-frequency_effects

use crate::{_impl_init, impl_trait};

/// Individual spatial audio channel positions.
///
/// These labels represent the canonical channel roles used in
/// multichannel layouts (L, R, C, LFE, surrounds, back channels).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioChannel {
    /// Left
    #[default]
    L,
    /// Right
    R,
    /// Center
    C,
    /// Low Frequency Effects
    LFE,
    /// Left Surround
    Ls,
    /// Right Surround
    Rs,
    /// Left Back
    Lb,
    /// Right Back
    Rb,
}
_impl_init![ConstInitCore: Self::L => AudioChannel];
impl_trait![fmt::Display for AudioChannel |self, f| f.write_str(self.as_code())];

impl AudioChannel {
    /// Returns the standard short code for this channel
    /// (L, R, C, LFE, Ls, Rs, Lb, Rb).
    ///
    /// These codes follow ITU naming and are commonly used
    /// in multichannel specifications and routing graphs.
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::L => "L",
            Self::R => "R",
            Self::C => "C",
            Self::LFE => "LFE",
            Self::Ls => "Ls",
            Self::Rs => "Rs",
            Self::Lb => "Lb",
            Self::Rb => "Rb",
        }
    }

    /// Returns a human-readable name for this channel,
    /// suitable for UI labels and descriptive text.
    pub const fn as_name(self) -> &'static str {
        match self {
            Self::L => "Left",
            Self::R => "Right",
            Self::C => "Center",
            Self::LFE => "Low Frequency Effects",
            Self::Ls => "Left Surround",
            Self::Rs => "Right Surround",
            Self::Lb => "Left Back",
            Self::Rb => "Right Back",
        }
    }
}

/// Fixed, discrete multichannel audio layouts (1.0, 2.0, 5.1, 7.1, …).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioChannels {
    /// Mono layout (1 channel).
    Mono = 0,

    /// Stereo layout: L, R.
    ///
    /// This is the default.
    #[default]
    Stereo,

    /// 3-channel layout: L, R, LFE.
    Stereo2_1,

    /// 3-channel layout: L, C, R.
    Surround3_0,

    /// 4-channel layout: L, R, Ls, Rs.
    Surround4_0,

    /// 5-channel layout: L, C, R, Ls, Rs.
    Surround5_0,

    /// 6-channel layout: L, C, R, Ls, Rs, LFE.
    Surround5_1,

    /// 7-channel surround layout.
    Surround7_0,

    /// 8-channel surround layout.
    Surround7_1,
}

_impl_init![ConstInitCore: Self::Stereo => AudioChannels];
impl_trait![fmt::Display for AudioChannels |self, f| f.write_str(self.as_x_y())];

impl AudioChannels {
    /// Creates a channel layout from the total number of channels and an `lfe` flag.
    ///
    /// Returns `None` if the combination is not a valid layout represented by this enum.
    pub const fn from_total_lfe(total: u8, lfe: bool) -> Option<Self> {
        match (total, lfe) {
            (1, false) => Some(Self::Mono),
            (2, false) => Some(Self::Stereo),
            (3, true) => Some(Self::Stereo2_1),
            (3, false) => Some(Self::Surround3_0),
            (4, false) => Some(Self::Surround4_0),
            (5, false) => Some(Self::Surround5_0),
            (6, true) => Some(Self::Surround5_1),
            (7, false) => Some(Self::Surround7_0),
            (8, true) => Some(Self::Surround7_1),
            _ => None,
        }
    }
    /// Creates a layout from `(main_channels, has_lfe)`
    /// following the standard `X.Y` surround notation.
    ///
    /// Returns `None` for unsupported combinations.
    pub const fn from_main_lfe(main: u8, lfe: bool) -> Option<Self> {
        match (main, lfe) {
            (1, false) => Some(Self::Mono),        // 1.0
            (2, false) => Some(Self::Stereo),      // 2.0
            (2, true) => Some(Self::Stereo2_1),    // 2.1
            (3, false) => Some(Self::Surround3_0), // 3.0
            (4, false) => Some(Self::Surround4_0), // 4.0
            (5, false) => Some(Self::Surround5_0), // 5.0
            (5, true) => Some(Self::Surround5_1),  // 5.1
            (7, false) => Some(Self::Surround7_0), // 7.0
            (7, true) => Some(Self::Surround7_1),  // 7.1
            _ => None,
        }
    }

    /* queries */

    /// Returns the total number of channels (main + LFE).
    pub const fn channels(self) -> u8 {
        match self {
            Self::Mono => 1,
            Self::Stereo => 2,
            Self::Stereo2_1 => 3,
            Self::Surround3_0 => 3,
            Self::Surround4_0 => 4,
            Self::Surround5_0 => 5,
            Self::Surround5_1 => 6,
            Self::Surround7_0 => 7,
            Self::Surround7_1 => 8,
        }
    }

    /// Returns the main number of channels.
    pub const fn main_channels(self) -> u8 {
        match self {
            Self::Mono => 1,
            Self::Stereo => 2,
            Self::Stereo2_1 => 2,
            Self::Surround3_0 => 3,
            Self::Surround4_0 => 4,
            Self::Surround5_0 => 5,
            Self::Surround5_1 => 5,
            Self::Surround7_0 => 7,
            Self::Surround7_1 => 7,
        }
    }

    /// Returns the main number of channels and whether it has LFE.
    pub const fn main_lfe(self) -> (u8, bool) {
        (self.main_channels(), self.has_lfe())
    }

    /// Whether the layout includes an LFE channel.
    pub const fn has_lfe(self) -> bool {
        matches![self, Self::Stereo2_1 | Self::Surround5_1 | Self::Surround7_1]
    }

    /// Distinguish basic stereo/mono from spatial layouts.
    ///
    /// Surround audio is any layout with loudspeakers placed around the listener,
    /// requiring 3 or more main channels.
    pub const fn is_surround(self) -> bool {
        !matches![self, Self::Mono | Self::Stereo | Self::Stereo2_1]
    }

    /// Whether the layout is stereo-based (2.x).
    pub const fn is_stereo_like(self) -> bool {
        matches![self, Self::Stereo | Self::Stereo2_1]
    }

    /// Returns the standard `X.Y` notation for this layout,
    /// where `X` is the number of main channels and `Y` is the
    /// number of LFE channels (0 or 1).
    ///
    /// This is the canonical format used across codecs, file
    /// containers, DAWs, and audio-engineering specifications.
    pub const fn as_x_y(self) -> &'static str {
        match self {
            Self::Mono => "1.0",
            Self::Stereo => "2.0",
            Self::Stereo2_1 => "2.1",
            Self::Surround3_0 => "3.0",
            Self::Surround4_0 => "4.0",
            Self::Surround5_0 => "5.0",
            Self::Surround5_1 => "5.1",
            Self::Surround7_0 => "7.0",
            Self::Surround7_1 => "7.1",
        }
    }

    /// Returns a human-readable name for this layout,
    /// suitable for UI labels and descriptive text.
    pub const fn as_name(self) -> &'static str {
        match self {
            Self::Mono => "Mono",
            Self::Stereo => "Stereo",
            Self::Stereo2_1 => "Stereo 2.1",
            Self::Surround3_0 => "3.0 surround",
            Self::Surround4_0 => "4.0 surround",
            Self::Surround5_0 => "5.0 surround",
            Self::Surround5_1 => "5.1 surround",
            Self::Surround7_0 => "7.0 surround",
            Self::Surround7_1 => "7.1 surround",
        }
    }

    /// Returns the canonical channel order for this layout as a static slice of channel labels.
    ///
    /// These labels follow the standard ordering used in surround formats
    /// and are independent of codec bitstream ordering.
    pub const fn channels_expanded(self) -> &'static [AudioChannel] {
        use AudioChannel::{C, L, LFE, Lb, Ls, R, Rb, Rs};
        match self {
            Self::Mono => &[L],
            Self::Stereo => &[L, R],
            Self::Stereo2_1 => &[L, R, LFE],
            Self::Surround3_0 => &[L, C, R],
            Self::Surround4_0 => &[L, R, Ls, Rs],
            Self::Surround5_0 => &[L, C, R, Ls, Rs],
            Self::Surround5_1 => &[L, C, R, Ls, Rs, LFE],
            Self::Surround7_0 => &[L, C, R, Ls, Rs, Lb, Rb],
            Self::Surround7_1 => &[L, C, R, Ls, Rs, Lb, Rb, LFE],
        }
    }
}
