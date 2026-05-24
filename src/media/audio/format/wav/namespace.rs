// devela::media::audio::format::wav::namespace
//
//! Defines [`PcmWav`].
//

#[cfg(feature = "alloc")]
use crate::Vec;
#[cfg(feature = "std")]
use crate::{Fs, Path};
use crate::{PcmWavBuf, PcmWavError, PcmWavFmt, Riff, is, unwrap};

#[doc = crate::_tags!(audio parser)]
/// Minimal WAVE parser for borrowed PCM-family audio.
#[doc = crate::_doc_location!("media/audio")]
#[derive(Debug)]
pub struct PcmWav;

impl PcmWav {
    /// Integer PCM.
    pub const FORMAT_PCM: u16 = 0x0001;
    /// IEEE floating-point PCM.
    pub const FORMAT_IEEE_FLOAT: u16 = 0x0003;

    /// Parses a borrowed WAVE byte region.
    ///
    /// This validates the RIFF container, finds the first `fmt ` chunk, finds
    /// the first `data` chunk, checks the basic PCM shape, and returns a
    /// byte-backed view into the original input.
    pub const fn parse(bytes: &[u8]) -> Result<PcmWavBuf<&[u8]>, PcmWavError> {
        let parts = unwrap![ok? Self::parse_parts(bytes)];
        Ok(PcmWavBuf::_new(bytes, parts.fmt, parts.data_offset, parts.data_len))
    }
    /// Alias for parser-style use from format modules.
    pub const fn decode(bytes: &[u8]) -> Result<PcmWavBuf<&[u8]>, PcmWavError> {
        Self::parse(bytes)
    }
    /// Parses owned WAVE bytes.
    #[cfg(feature = "alloc")]
    pub fn from_vec(bytes: Vec<u8>) -> Result<PcmWavBuf<Vec<u8>>, PcmWavError> {
        let parts = Self::parse_parts(bytes.as_slice())?;
        Ok(PcmWavBuf::_new(bytes, parts.fmt, parts.data_offset, parts.data_len))
    }
    /// Reads and parses a WAVE file.
    #[cfg(feature = "std")]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<PcmWavBuf<Vec<u8>>, PcmWavError> {
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

    const fn parse_parts(bytes: &[u8]) -> Result<PcmWavParts, PcmWavError> {
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
        let mut data_len = 0usize;
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
                if data_len == 0 && data_offset == 0 {
                    data_len = chunk.data().len();
                    // Full WAVE byte layout:
                    // - RIFF header:       8 bytes
                    // - WAVE form tag:     4 bytes
                    // - subchunk header:   8 bytes
                    //
                    // `chunk.offset()` is relative to the subchunk region, which
                    // starts after the RIFF header and WAVE form tag.
                    let Some(offset) = 12usize.checked_add(chunk.offset()) else {
                        return Err(PcmWavError::Overflow);
                    };
                    let Some(offset) = offset.checked_add(8) else {
                        return Err(PcmWavError::Overflow);
                    };
                    data_offset = offset;
                }
            }
            is! { fmt.is_some() && (data_len != 0 || data_offset != 0), break }
        }
        let Some(fmt) = fmt else {
            return Err(PcmWavError::MissingFmt);
        };
        is! { data_len == 0 && data_offset == 0, return Err(PcmWavError::MissingData) }
        match Self::validate_data_shape(fmt, data_len) {
            Ok(()) => Ok(PcmWavParts { fmt, data_offset, data_len }),
            Err(err) => Err(err),
        }
    }

    /// Checks that the `data` chunk length is frame-aligned.
    const fn validate_data_shape(fmt: PcmWavFmt, data_len: usize) -> Result<(), PcmWavError> {
        is! { fmt.block_align == 0, return Err(PcmWavError::InvalidBlockAlign) }
        if !data_len.is_multiple_of(fmt.block_align as usize) {
            return Err(PcmWavError::InvalidDataLength);
        }
        Ok(())
    }
    /// Returns the packed byte width for the supported WAVE sample encoding.
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

#[derive(Clone, Copy)]
struct PcmWavParts {
    fmt: PcmWavFmt,
    data_offset: usize,
    data_len: usize,
}

const fn read_u16_le(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}
const fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
}
