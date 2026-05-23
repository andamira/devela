// devela::media::audio::pcm::format::wav::fmt_ref
//
//! Defines [`PcmWavRef`], [`PcmWavAlloc`].
//

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{AudioChannels, PcmSample, PcmSpec, PcmWav, PcmWavError, PcmWavFmt};

#[doc = crate::_tags!(audio)]
/// Borrowed WAVE container view.
#[doc = crate::_doc_location!("media/audio")]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PcmWavRef<'a> {
    fmt: PcmWavFmt,
    data: &'a [u8],
    data_offset: usize,
}
#[rustfmt::skip]
impl<'a> PcmWavRef<'a> {
    pub(super) const fn _new(fmt: PcmWavFmt, data: &'a [u8], data_offset: usize) -> Self {
        Self { fmt, data, data_offset }
    }
    /// Returns the parsed `fmt ` metadata.
    pub const fn fmt(self) -> PcmWavFmt { self.fmt }

    /// Returns the borrowed `data` chunk bytes.
    pub const fn data_bytes(self) -> &'a [u8] { self.data }
    /// Returns the byte offset of the `data` payload in the original WAVE bytes.
    pub const fn data_offset(self) -> usize { self.data_offset }
    /// Returns the byte length of the `data` payload.
    pub const fn data_len(self) -> usize { self.data.len() }
    /// Returns the byte offset and byte length of the `data` payload.
    pub const fn data_span(self) -> (usize, usize) { (self.data_offset, self.data.len()) }

    /// Returns the number of interleaved frames.
    pub const fn frames(self) -> usize {
        self.data.len() / self.fmt.block_align as usize
    }
    /// Maps this WAVE view to current [`PcmSpec`] metadata when unambiguous.
    pub const fn spec(self) -> Result<PcmSpec, PcmWavError> {
        let sample = match (self.fmt.format_tag, self.fmt.bits_per_sample) {
            (PcmWav::FORMAT_PCM, 8) => PcmSample::U8,
            (PcmWav::FORMAT_PCM, 16) => PcmSample::I16,
            (PcmWav::FORMAT_PCM, 24) => PcmSample::I24,
            (PcmWav::FORMAT_PCM, 32) => PcmSample::I32,
            (PcmWav::FORMAT_IEEE_FLOAT, 32) => PcmSample::F32,
            (PcmWav::FORMAT_IEEE_FLOAT, 64) => PcmSample::F64,
            (_, bits) => return Err(PcmWavError::UnsupportedBitsPerSample(bits)),
        };
        let channels = match self.fmt.channels {
            1 => AudioChannels::Mono,
            2 => AudioChannels::Stereo,
            other => return Err(PcmWavError::UnsupportedChannelCount(other)),
        };
        Ok(PcmSpec::new(sample, channels, self.fmt.sample_rate))
    }
}

#[cfg(feature = "alloc")]
#[doc = crate::_tags!(audio)]
/// Allocated WAVE bytes with parsed PCM metadata.
#[doc = crate::_doc_location!("media/audio")]
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PcmWavAlloc {
    bytes: Vec<u8>,
    fmt: PcmWavFmt,
    data_offset: usize,
    data_len: usize,
}

#[cfg(feature = "alloc")]
#[rustfmt::skip]
impl PcmWavAlloc {
    pub(super) fn _new(bytes: Vec<u8>, fmt: PcmWavFmt, data_offset: usize, data_len: usize)
        -> Self { Self { bytes, fmt, data_offset, data_len } }

    /// Parses owned WAVE bytes.
    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, PcmWavError> {
        let wav = PcmWav::parse(&bytes)?;
        let fmt = wav.fmt();
        let data_offset = wav.data_offset();
        let data_len = wav.data_len();
        Ok(Self::_new(bytes, fmt, data_offset, data_len))
    }
    /// Returns the full owned WAVE byte region.
    pub fn bytes(&self) -> &[u8] { &self.bytes }
    /// Returns the owned bytes.
    pub fn into_bytes(self) -> Vec<u8> { self.bytes }

    /// Returns this owned WAVE as a borrowed view.
    pub fn as_ref(&self) -> PcmWavRef<'_> {
        let end = self.data_offset + self.data_len;
        PcmWavRef::_new(self.fmt, &self.bytes[self.data_offset..end], self.data_offset)
    }
    /// Returns the parsed `fmt ` metadata.
    pub const fn fmt(&self) -> PcmWavFmt { self.fmt }
    /// Returns the borrowed `data` chunk bytes.
    pub fn data_bytes(&self) -> &[u8] { self.as_ref().data_bytes() }
    /// Returns the byte offset of the `data` payload in the full WAVE byte slice.
    pub const fn data_offset(&self) -> usize { self.data_offset }
    /// Returns the byte length of the `data` payload.
    pub const fn data_len(&self) -> usize { self.data_len }
    /// Returns the byte offset and length of the `data` payload.
    pub const fn data_span(&self) -> (usize, usize) { (self.data_offset, self.data_len) }

    /// Returns the number of interleaved frames.
    pub const fn frames(&self) -> usize { self.data_len / self.fmt.block_align as usize }
    /// Maps this WAVE view to current [`PcmSpec`] metadata when unambiguous.
    pub const fn spec(&self) -> Result<PcmSpec, PcmWavError> {
        PcmWavRef::_new(self.fmt, &[], self.data_offset).spec()
    }
}
