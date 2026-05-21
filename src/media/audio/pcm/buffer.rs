// devela::media::audio::pcm::buffer
//
//! Defines [`PcmBuffer`], [`PcmPlanar`].
//

use crate::{AudioChannels, ConstInit, PcmSample, PcmSpec};
use crate::{is, test_size_of, whilst};

test_size_of![pcm_buffer8: PcmBuffer<'_, i8> = 24]; // 192 bits
test_size_of![pcm_buffer16: PcmBuffer<'_, i16> = 24]; // 192 bits
test_size_of![pcm_buffer32: PcmBuffer<'_, f32> = 24]; // 192 bits
test_size_of![pcm_buffer64: PcmBuffer<'_, f64> = 24]; // 192 bits
#[doc = crate::_tags!(audio)]
/// Borrowed interleaved PCM samples: LRLRLR…
#[doc = crate::_doc_location!("media/audio")]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PcmBuffer<'a, T> {
    /// Interleaved samples.
    pub data: &'a [T],
    /// Sample encoding.
    pub format: PcmSample,
    /// Channel layout.
    pub channels: AudioChannels,
    /// Samples per second, in Hz.
    pub sample_rate: u32,
}
impl<'a, T> ConstInit for PcmBuffer<'a, T> {
    const INIT: Self = Self::new(&[], PcmSample::INIT, AudioChannels::INIT, 0);
}
impl<'a, T> PcmBuffer<'a, T> {
    /// Creates a borrowed interleaved PCM buffer.
    pub const fn new(
        data: &'a [T],
        format: PcmSample,
        channels: AudioChannels,
        sample_rate: u32,
    ) -> Self {
        Self { data, format, channels, sample_rate }
    }
    /// Returns the sample format.
    #[must_use]
    pub const fn format(&self) -> PcmSample {
        self.format
    }
    /// Returns the channel layout.
    #[must_use]
    pub const fn channels(&self) -> AudioChannels {
        self.channels
    }
    /// Returns the number of channels.
    #[must_use]
    pub const fn channel_count(&self) -> usize {
        self.channels.channels() as usize
    }
    /// Returns the sample rate in Hertz.
    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    /// Returns the stream metadata.
    #[must_use]
    pub const fn spec(&self) -> PcmSpec {
        PcmSpec::new(self.format, self.channels, self.sample_rate)
    }
    /// Returns whether there are no samples.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Returns the number of sample values.
    #[must_use]
    pub const fn samples_len(&self) -> usize {
        self.data.len()
    }
    /// Returns the frame count, or `None` if the sample count is incomplete.
    #[must_use]
    pub const fn frames(&self) -> Option<usize> {
        let (len, channels) = (self.samples_len(), self.channel_count());
        is! { channels == 0 || len % channels != 0, None, Some(len / channels) }
    }
    /// Returns whether the sample data contains complete frames.
    #[must_use]
    pub const fn has_complete_frames(&self) -> bool {
        self.frames().is_some()
    }
}

test_size_of![pcm_planar: PcmPlanar<'_, f32> = 24]; // 192 bits
#[doc = crate::_tags!(audio)]
/// Planar PCM audio buffer: `[LLLL…], [RRRR…]`.
#[doc = crate::_doc_location!("media/audio")]
#[must_use]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PcmPlanar<'a, T> {
    /// Per-channel sample planes, in channel layout order.
    pub planes: &'a [&'a [T]],
    /// Sample encoding.
    pub format: PcmSample,
    /// Channel layout.
    pub channels: AudioChannels,
    /// Samples per second, in Hz.
    pub sample_rate: u32,
}
impl<'a, T> ConstInit for PcmPlanar<'a, T> {
    const INIT: Self = Self::new(&[], PcmSample::INIT, AudioChannels::INIT, 0);
}
impl<'a, T> PcmPlanar<'a, T> {
    /// Creates a borrowed planar PCM buffer.
    pub const fn new(
        planes: &'a [&'a [T]],
        format: PcmSample,
        channels: AudioChannels,
        sample_rate: u32,
    ) -> Self {
        Self { planes, format, channels, sample_rate }
    }
    /// Returns the sample format.
    #[must_use]
    pub const fn format(&self) -> PcmSample {
        self.format
    }
    /// Returns the channel layout.
    #[must_use]
    pub const fn channels(&self) -> AudioChannels {
        self.channels
    }
    /// Returns the number of channels.
    #[must_use]
    pub const fn channel_count(&self) -> usize {
        self.channels.channels() as usize
    }
    /// Returns the sample rate in Hertz.
    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    /// Returns the stream metadata.
    #[must_use]
    pub const fn spec(&self) -> PcmSpec {
        PcmSpec::new(self.format, self.channels, self.sample_rate)
    }
    /// Returns whether there are no planes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.planes.is_empty()
    }
    /// Returns the number of planes.
    #[must_use]
    pub const fn planes_len(&self) -> usize {
        self.planes.len()
    }
    /// Returns the frame count, or `None` if planes are incomplete or uneven.
    #[must_use]
    pub const fn frames(&self) -> Option<usize> {
        let planes = self.planes.len();
        is! { planes != self.channel_count(), return None }
        is! { planes == 0, return Some(0) }
        let frames = self.planes[0].len();
        whilst! { i in 1..planes; {
            is! { self.planes[i].len() != frames, return None }
        }}
        Some(frames)
    }
    /// Returns whether the plane data contains complete frames.
    #[must_use]
    pub const fn has_complete_frames(&self) -> bool {
        self.frames().is_some()
    }
}
