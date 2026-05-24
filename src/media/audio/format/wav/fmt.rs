// devela::media::audio::format::wav::fmt
//
//! Defines [`PcmWavFmt`].
//

use crate::{AudioChannels, PcmSample, PcmSpec, PcmWav, PcmWavError};

#[doc = crate::_tags!(audio parser)]
/// Parsed WAVE `fmt` chunk.
#[doc = crate::_doc_location!("media/audio")]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PcmWavFmt {
    /// WAVE format code.
    pub format_tag: u16,
    /// Number of interleaved channels.
    pub channels: u16,
    /// Samples per second.
    pub sample_rate: u32,
    /// Average bytes per second.
    pub byte_rate: u32,
    /// Bytes per interleaved frame.
    pub block_align: u16,
    /// Meaningful bits per sample.
    pub bits_per_sample: u16,
}
impl PcmWavFmt {
    /// Maps this WAVE format metadata to current [`PcmSpec`] metadata when unambiguous.
    pub const fn spec(self) -> Result<PcmSpec, PcmWavError> {
        let sample = match (self.format_tag, self.bits_per_sample) {
            (PcmWav::FORMAT_PCM, 8) => PcmSample::U8,
            (PcmWav::FORMAT_PCM, 16) => PcmSample::I16,
            (PcmWav::FORMAT_PCM, 24) => PcmSample::I24,
            (PcmWav::FORMAT_PCM, 32) => PcmSample::I32,
            (PcmWav::FORMAT_IEEE_FLOAT, 32) => PcmSample::F32,
            (PcmWav::FORMAT_IEEE_FLOAT, 64) => PcmSample::F64,
            (_, bits) => return Err(PcmWavError::UnsupportedBitsPerSample(bits)),
        };
        let channels = match self.channels {
            1 => AudioChannels::Mono,
            2 => AudioChannels::Stereo,
            other => return Err(PcmWavError::UnsupportedChannelCount(other)),
        };
        Ok(PcmSpec::new(sample, channels, self.sample_rate))
    }
}
