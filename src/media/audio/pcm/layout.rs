// devela::media::audio::pcm::layout
//
//! Defines [`PcmLayout`].
//

#[cfg(feature = "alsa")]
use crate::_alsa_raw;
use crate::{_impl_init, impl_trait};

#[doc = crate::_tags!(audio)]
/// Sample arrangement of a PCM buffer.
#[doc = crate::_doc_location!("media/audio")]
///
/// A PCM layout describes how channel samples are placed in memory.
///
/// It is independent from [`PcmSpec`]: the same sample type, channel layout,
/// and sample rate can be represented as either interleaved or planar data.
///
/// [`PcmSpec`]: crate::PcmSpec
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PcmLayout {
    /// Channel samples are mixed by frame: `LRLRLR…`.
    #[default]
    Interleaved,

    /// Channel samples are stored in separate planes: `[LLLL…], [RRRR…]`.
    Planar,
}

_impl_init![Self::Interleaved => PcmLayout];

impl PcmLayout {
    /// Returns a compact stable code for this layout.
    #[must_use]
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::Interleaved => "interleaved",
            Self::Planar => "planar",
        }
    }
    /// Returns a human-readable name for this layout.
    #[must_use]
    pub const fn as_name(self) -> &'static str {
        match self {
            Self::Interleaved => "Interleaved",
            Self::Planar => "Planar",
        }
    }
    /// Returns whether this layout stores all channel samples in one mixed slice.
    #[must_use]
    pub const fn is_interleaved(self) -> bool {
        matches!(self, Self::Interleaved)
    }
    /// Returns whether this layout stores each channel in its own sample plane.
    #[must_use]
    pub const fn is_planar(self) -> bool {
        matches!(self, Self::Planar)
    }

    // Converts the sample to ALSA access.
    #[cfg(feature = "alsa")]
    pub(crate) const fn to_alsa(self) -> _alsa_raw::snd_pcm_access_t {
        match self {
            Self::Interleaved => _alsa_raw::SND_PCM_ACCESS_RW_INTERLEAVED,
            Self::Planar => _alsa_raw::SND_PCM_ACCESS_RW_NONINTERLEAVED,
        }
    }
}

impl_trait![fmt::Display for PcmLayout |self, f| f.write_str(self.as_code())];
