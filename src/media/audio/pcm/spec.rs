// devela::media::audio::pcm::spec
//
//! Defines [`PcmSpec`].
//

use crate::{_impl_init, AudioChannels, PcmSample, impl_trait};

#[doc = crate::_tags!(audio)]
/// Essential metadata describing a PCM audio stream.
#[doc = crate::_doc_meta!{
    location("media/audio"),
    test_size_of(PcmSpec = 8|64),
}]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PcmSpec {
    /// bit-depth + numeric type
    pub sample: PcmSample,
    /// layout (Mono, Stereo, 5.1…)
    pub channels: AudioChannels,
    /// Hz
    pub sample_rate: u32,
}
_impl_init![Self::new(PcmSample::INIT, AudioChannels::INIT, 0) => PcmSpec];
impl_trait![fmt::Display for PcmSpec |self, f| {
    write!(f, "{}/{}@{}Hz", self.sample, self.channels, self.sample_rate)
}];

impl PcmSpec {
    /// Creates a PCM stream specification.
    #[must_use]
    pub const fn new(sample: PcmSample, channels: AudioChannels, sample_rate: u32) -> Self {
        Self { sample, channels, sample_rate }
    }
    /// Returns the number of channels.
    #[must_use]
    pub const fn channel_count(self) -> usize {
        self.channels.channels() as usize
    }
    /// Returns the byte size of one interleaved frame.
    #[must_use]
    pub const fn frame_bytes(self) -> usize {
        self.sample.bytes() * self.channel_count()
    }
    /// Returns whether the sample rate is non-zero.
    #[must_use]
    pub const fn has_valid_rate(self) -> bool {
        self.sample_rate != 0
    }
    /// Returns whether this stream has a non-zero channel count and sample rate.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.channel_count() != 0 && self.sample_rate != 0 && self.frame_bytes() != 0
    }
    /// Returns the number of frames in `data_len`, if frame-aligned.
    pub const fn frames_for_data_len(self, data_len: usize) -> Option<usize> {
        let frame_bytes = self.frame_bytes();
        if frame_bytes == 0 || !data_len.is_multiple_of(frame_bytes) {
            None
        } else {
            Some(data_len / frame_bytes)
        }
    }
    /// Returns whether `data_len` contains complete interleaved frames.
    #[must_use]
    pub const fn has_complete_frames_for_data_len(self, data_len: usize) -> bool {
        self.frames_for_data_len(data_len).is_some()
    }
}
