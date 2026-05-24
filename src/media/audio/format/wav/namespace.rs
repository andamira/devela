// devela::media::audio::format::wav::namespace
//
//! Defines [`PcmWavFmt`], [`PcmWav`].
//

#[cfg(feature = "std")]
use crate::{Fs, Path};
#[cfg(feature = "alloc")]
use crate::{PcmWavAlloc, Vec};
use crate::{PcmWavError, PcmWavRef, Riff, is};

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

#[doc = crate::_tags!(audio)]
/// Minimal WAVE parser for borrowed PCM-family audio.
#[doc = crate::_doc_location!("media/audio")]
#[derive(Debug)]
pub struct PcmWav;

impl PcmWav {
    /// Integer PCM.
    pub const FORMAT_PCM: u16 = 0x0001;

    /// IEEE floating-point PCM.
    pub const FORMAT_IEEE_FLOAT: u16 = 0x0003;

    /// Parses a borrowed WAVE container.
    pub const fn parse(bytes: &[u8]) -> Result<PcmWavRef<'_>, PcmWavError> {
        let root = match Riff::root(bytes) {
            Ok(root) => root,
            Err(err) => return Err(PcmWavError::Riff(err)),
        };
        match root.container_type() {
            Some(tag) if tag.eq(Riff::WAVE) => {}
            _ => return Err(PcmWavError::NotWave),
        }
        let mut chunks = match root.subchunks() {
            Ok(chunks) => chunks,
            Err(err) => return Err(PcmWavError::Riff(err)),
        };
        let mut fmt: Option<PcmWavFmt> = None;
        let mut data: Option<&[u8]> = None;
        let mut data_offset = 0usize;
        while let Some(next) = chunks.next_chunk() {
            let chunk = match next {
                Ok(chunk) => chunk,
                Err(err) => return Err(PcmWavError::Riff(err)),
            };
            if chunk.id().eq(Riff::FMT) {
                if fmt.is_none() {
                    fmt = match Self::parse_fmt(chunk.data()) {
                        Ok(fmt) => Some(fmt),
                        Err(err) => return Err(err),
                    };
                }
            } else if chunk.id().eq(Riff::DATA) {
                if data.is_none() {
                    data = Some(chunk.data());
                    // Root RIFF header: 8 bytes.
                    // WAVE form tag:    4 bytes.
                    // Chunk header:     8 bytes.
                    let Some(offset) = 12usize.checked_add(chunk.offset()) else {
                        return Err(PcmWavError::Overflow);
                    };
                    let Some(offset) = offset.checked_add(8) else {
                        return Err(PcmWavError::Overflow);
                    };
                    data_offset = offset;
                }
            }
            is! { fmt.is_some() && data.is_some(), break }
        }
        let Some(fmt) = fmt else {
            return Err(PcmWavError::MissingFmt);
        };
        let Some(data) = data else {
            return Err(PcmWavError::MissingData);
        };
        let wav = PcmWavRef::_new(fmt, data, data_offset);
        match Self::validate_data_shape(wav) {
            Ok(()) => Ok(wav),
            Err(err) => Err(err),
        }
    }
    /// Alias for parser-style use from format modules.
    pub const fn decode(bytes: &[u8]) -> Result<PcmWavRef<'_>, PcmWavError> {
        Self::parse(bytes)
    }

    /// Parses an owned WAVE container.
    #[cfg(feature = "alloc")]
    pub fn from_vec(bytes: Vec<u8>) -> Result<PcmWavAlloc, PcmWavError> {
        PcmWavAlloc::from_vec(bytes)
    }

    /// Reads and parses a WAVE file.
    #[cfg(feature = "std")]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<PcmWavAlloc, PcmWavError> {
        Self::from_vec(Fs::read(path)?)
    }

    /// Parses the base WAVE `fmt` payload.
    pub const fn parse_fmt(bytes: &[u8]) -> Result<PcmWavFmt, PcmWavError> {
        is! { bytes.len() < 16, return Err(PcmWavError::TruncatedFmt) }
        let fmt = PcmWavFmt {
            format_tag: read_u16_le(bytes, 0),
            channels: read_u16_le(bytes, 2),
            sample_rate: read_u32_le(bytes, 4),
            byte_rate: read_u32_le(bytes, 8),
            block_align: read_u16_le(bytes, 12),
            bits_per_sample: read_u16_le(bytes, 14),
        };
        match fmt.format_tag {
            Self::FORMAT_PCM | Self::FORMAT_IEEE_FLOAT => {}
            other => return Err(PcmWavError::UnsupportedFormat(other)),
        }
        if fmt.channels == 0 {
            return Err(PcmWavError::UnsupportedChannelCount(fmt.channels));
        }
        let bytes_per_sample = match Self::bytes_per_sample(fmt.format_tag, fmt.bits_per_sample) {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };
        let Some(expected_align) = fmt.channels.checked_mul(bytes_per_sample) else {
            return Err(PcmWavError::InvalidBlockAlign);
        };
        is! { fmt.block_align != expected_align, return Err(PcmWavError::InvalidBlockAlign) }
        let Some(expected_rate) = fmt.sample_rate.checked_mul(fmt.block_align as u32) else {
            return Err(PcmWavError::InvalidByteRate);
        };
        is! { fmt.byte_rate != expected_rate, return Err(PcmWavError::InvalidByteRate) }

        Ok(fmt)
    }

    /* private helpers */

    const fn validate_data_shape(wav: PcmWavRef<'_>) -> Result<(), PcmWavError> {
        is! { wav.fmt().block_align == 0, return Err(PcmWavError::InvalidBlockAlign) }
        if !wav.data_len().is_multiple_of(wav.fmt().block_align as usize) {
            return Err(PcmWavError::InvalidDataLength);
        }
        Ok(())
    }
    const fn bytes_per_sample(format_tag: u16, bits: u16) -> Result<u16, PcmWavError> {
        match (format_tag, bits) {
            (Self::FORMAT_PCM, 8) => Ok(1),
            (Self::FORMAT_PCM, 16) => Ok(2),
            (Self::FORMAT_PCM, 24) => Ok(3),
            (Self::FORMAT_PCM, 32) => Ok(4),
            (Self::FORMAT_IEEE_FLOAT, 32) => Ok(4),
            (Self::FORMAT_IEEE_FLOAT, 64) => Ok(8),
            (_, other) => Err(PcmWavError::UnsupportedBitsPerSample(other)),
        }
    }
}
const fn read_u16_le(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}
const fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
}
