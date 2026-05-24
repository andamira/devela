// devela::media::audio::format::wav::fmt
//
//! Defines [`PcmWavFmt`].
//

use crate::{AudioChannels, PcmSample, PcmSpec, PcmWav, PcmWavError, is};

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
    /// Number of extension bytes after the base 16-byte format.
    ///
    /// This is `0` for the classic 16-byte PCM form. When serialized as a
    /// WAVEFORMATEX chunk, the `cbSize` field itself occupies two bytes and
    /// contains this value.
    pub extra_len: u16,
}
impl PcmWavFmt {
    /// The base PCM `fmt ` payload length.
    pub const BASE_LEN: usize = 16;
    /// The WAVEFORMATEX `fmt ` payload length with `cbSize = 0`.
    pub const WAVEFORMATEX_EMPTY_LEN: usize = 18;

    /// Creates validated WAVE format metadata from a PCM stream specification.
    pub const fn from_spec(spec: PcmSpec) -> Result<Self, PcmWavError> {
        let format_tag = if spec.sample.is_float() {
            PcmWav::FORMAT_IEEE_FLOAT
        } else if spec.sample.is_int() {
            PcmWav::FORMAT_PCM
        } else {
            return Err(PcmWavError::UnsupportedBitsPerSample(spec.sample.bits() as u16));
        };
        Self::new(
            format_tag,
            spec.channel_count() as u16,
            spec.sample_rate,
            spec.sample.bits() as u16,
            if format_tag == PcmWav::FORMAT_IEEE_FLOAT { 0 } else { 0 },
        )
    }
    /// Creates validated WAVE format metadata.
    pub const fn new(
        format_tag: u16,
        channels: u16,
        sample_rate: u32,
        bits_per_sample: u16,
        extra_len: u16,
    ) -> Result<Self, PcmWavError> {
        match format_tag {
            PcmWav::FORMAT_PCM | PcmWav::FORMAT_IEEE_FLOAT => {}
            other => return Err(PcmWavError::UnsupportedFormat(other)),
        }
        is! { channels == 0, return Err(PcmWavError::UnsupportedChannelCount(channels)) }
        let bytes_per_sample = match PcmWav::bytes_per_sample(format_tag, bits_per_sample) {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };
        let Some(block_align) = channels.checked_mul(bytes_per_sample) else {
            return Err(PcmWavError::InvalidBlockAlign);
        };
        let Some(byte_rate) = sample_rate.checked_mul(block_align as u32) else {
            return Err(PcmWavError::InvalidByteRate);
        };
        Ok(Self {
            format_tag,
            channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
            extra_len,
        })
    }
    /// Returns the encoded `fmt ` chunk payload length.
    ///
    /// PCM is written as the classic 16-byte `fmt ` form.
    /// IEEE float is written as WAVEFORMATEX with `cbSize = 0`.
    pub const fn encoded_len(self) -> usize {
        if self.format_tag == PcmWav::FORMAT_PCM && self.extra_len == 0 {
            Self::BASE_LEN
        } else {
            Self::WAVEFORMATEX_EMPTY_LEN + self.extra_len as usize
        }
    }
    /// Returns whether this format should be accompanied by a `fact` chunk.
    #[must_use]
    pub const fn needs_fact(self) -> bool {
        self.format_tag != PcmWav::FORMAT_PCM
    }
    /// Returns the number of frames in `data_len`, if frame-aligned.
    pub const fn frames_for_data_len(self, data_len: usize) -> Result<usize, PcmWavError> {
        is![self.block_align == 0, return Err(PcmWavError::InvalidBlockAlign)];
        is![data_len % self.block_align as usize != 0, return Err(PcmWavError::InvalidDataLength)];
        Ok(data_len / self.block_align as usize)
    }
    /// Validates this format against its derived fields.
    pub const fn validate(self) -> Result<Self, PcmWavError> {
        is![self.channels == 0, return Err(PcmWavError::UnsupportedChannelCount(self.channels))];
        let bytes_per_sample = match PcmWav::bytes_per_sample(self.format_tag, self.bits_per_sample)
        {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };
        let expected_align = match self.channels.checked_mul(bytes_per_sample) {
            Some(v) => v,
            None => return Err(PcmWavError::InvalidBlockAlign),
        };
        if self.block_align != expected_align {
            return Err(PcmWavError::InvalidBlockAlign);
        }
        let expected_rate = match self.sample_rate.checked_mul(self.block_align as u32) {
            Some(v) => v,
            None => return Err(PcmWavError::InvalidByteRate),
        };
        is![self.byte_rate != expected_rate, return Err(PcmWavError::InvalidByteRate)];
        Ok(self)
    }
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
