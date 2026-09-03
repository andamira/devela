// devela/src/media/audio/pcm/spec.rs
//
//! Defines [`PcmSpec`].
//

use crate::{_impl_init, AudioChannels, PcmSample, RatioU32, impl_trait, is, unwrap};

#[doc = crate::_tags!(audio)]
/// Essential metadata describing a PCM audio stream.
#[doc = crate::_doc_meta!{
    location("media/audio"),
    test_size_of(PcmSpec = 8|64),
}]
/// # Example
/// ```
/// use devela::{AudioChannels, PcmSample, PcmSpec, RatioU32};
///
/// let spec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 44_101);
///
/// assert!(spec.is_valid());
/// assert_eq![spec.channel_count(), 2];
/// assert_eq![spec.frame_bytes(), 4];
/// assert_eq![spec.frames_for_data_len(16), Some(4)];
///
/// assert_eq![spec.nyquist_hz_ratio(), RatioU32::new(44_101, 2)];
/// assert_eq![spec.nyquist_hz_f64(), Some(22_050.5)];
/// assert_eq![spec.sample_period_secs_ratio(), RatioU32::new(1, 44_101)];
/// ```
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

impl PcmSpec {
    /// Returns the exact Nyquist frequency in hertz.
    #[must_use]
    pub const fn nyquist_hz_ratio(self) -> Option<RatioU32> {
        is! { self.sample_rate == 0, None,
        unwrap![some_map RatioU32::new(self.sample_rate, 2), |r| r.reduced()] }
    }
    /// Returns the Nyquist frequency in hertz as `f64`.
    #[must_use]
    pub const fn nyquist_hz_f64(self) -> Option<f64> {
        is! { self.sample_rate == 0, None, Some(self.sample_rate as f64 / 2.0) }
    }
    /// Returns the exact duration of one sample period in seconds.
    #[must_use]
    pub const fn sample_period_secs_ratio(self) -> Option<RatioU32> {
        is! { self.sample_rate == 0, None, RatioU32::new(1, self.sample_rate) }
    }
    /// Returns the duration of one sample period in seconds as `f64`.
    #[must_use]
    pub const fn sample_period_secs_f64(self) -> Option<f64> {
        is! { self.sample_rate == 0, None, Some(1.0 / self.sample_rate as f64) }
    }
}
