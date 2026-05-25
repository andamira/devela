// devela::media::audio::format::wav::namespace
//
//! Defines [`PcmWav`].
//

#[cfg(feature = "std")]
use crate::{Fs, Path};
#[cfg(feature = "alloc")]
use crate::{PcmSpec, Vec, vec_};
use crate::{PcmWavBuf, PcmWavError, PcmWavFmt, Riff, is, slice, unwrap, whilst};

#[doc = crate::_tags!(audio parser)]
/// WAVE operations for PCM-family audio.
#[doc = crate::_doc_location!("media/audio")]
#[derive(Debug)]
pub struct PcmWav;

// Decoding methods
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
        is! { bytes.len() < PcmWavFmt::BASE_LEN, return Err(PcmWavError::TruncatedFmt) }
        let extra_len = if bytes.len() >= PcmWavFmt::WAVEFORMATEX_EMPTY_LEN {
            read_u16_le(bytes, 16)
        } else {
            0
        };
        let fmt = PcmWavFmt {
            format_tag: read_u16_le(bytes, 0),
            channels: read_u16_le(bytes, 2),
            sample_rate: read_u32_le(bytes, 4),
            byte_rate: read_u32_le(bytes, 8),
            block_align: read_u16_le(bytes, 12),
            bits_per_sample: read_u16_le(bytes, 14),
            extra_len,
        };
        fmt.validate()
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
    pub(super) const fn bytes_per_sample(format_tag: u16, bits: u16) -> Result<u16, PcmWavError> {
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
// Encoding methods
impl PcmWav {
    /// Returns the encoded WAVE byte length for `fmt` and `data`.
    ///
    /// This writes a minimal RIFF/WAVE container with:
    /// - one `fmt ` chunk,
    /// - one optional `fact` chunk for non-PCM formats,
    /// - one contiguous `data` chunk.
    pub const fn encoded_len(fmt: PcmWavFmt, data_len: usize) -> Result<usize, PcmWavError> {
        let fmt = match fmt.validate() {
            Ok(fmt) => fmt,
            Err(err) => return Err(err),
        };
        match fmt.frames_for_data_len(data_len) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
        is! { !Riff::fits_u32(data_len), return Err(PcmWavError::SizeOutOfRange) }
        let Some(fmt_chunk_len) = Riff::chunk_len(fmt.encoded_len()) else {
            return Err(PcmWavError::Overflow);
        };
        let fact_chunk_len = if fmt.needs_fact() {
            match Riff::chunk_len(4) {
                Some(len) => len,
                None => return Err(PcmWavError::Overflow),
            }
        } else {
            0
        };
        let Some(data_chunk_len) = Riff::chunk_len(data_len) else {
            return Err(PcmWavError::Overflow);
        };
        let Some(subchunks_len) = fmt_chunk_len.checked_add(fact_chunk_len) else {
            return Err(PcmWavError::Overflow);
        };
        let Some(subchunks_len) = subchunks_len.checked_add(data_chunk_len) else {
            return Err(PcmWavError::Overflow);
        };
        // The RIFF size field stores `4 + subchunks_len`.
        let Some(riff_data_len) = Riff::FORM_TYPE_LEN.checked_add(subchunks_len) else {
            return Err(PcmWavError::Overflow);
        };
        is! { !Riff::fits_u32(riff_data_len), return Err(PcmWavError::SizeOutOfRange) }
        match Riff::form_len(subchunks_len) {
            Some(len) => Ok(len),
            None => Err(PcmWavError::Overflow),
        }
    }
    /// Encodes a minimal PCM-family WAVE file into `dst`.
    ///
    /// Returns the number of bytes written.
    pub const fn encode_into(
        dst: &mut [u8],
        fmt: PcmWavFmt,
        data: &[u8],
    ) -> Result<usize, PcmWavError> {
        let fmt = unwrap![ok? fmt.validate()];
        let frames = unwrap![ok? fmt.frames_for_data_len(data.len())];
        let len = unwrap![ok? Self::encoded_len(fmt, data.len())];
        is! { dst.len() < len, return Err(PcmWavError::NotEnoughSpace) }
        is! { frames > u32::MAX as usize, return Err(PcmWavError::SizeOutOfRange) }
        let fmt_len = fmt.encoded_len();
        let fmt_chunk_len = unwrap![some_ok_or? Riff::chunk_len(fmt_len), PcmWavError::Overflow];
        let fact_chunk_len = if fmt.needs_fact() {
            unwrap![some_ok_or? Riff::chunk_len(4), PcmWavError::Overflow]
        } else {
            0
        };
        let data_chunk_len =
            unwrap![some_ok_or? Riff::chunk_len(data.len()), PcmWavError::Overflow];
        let subchunks_len = unwrap![some_ok_or?
            fmt_chunk_len.checked_add(fact_chunk_len), PcmWavError::Overflow];
        let subchunks_len = unwrap![some_ok_or?
            subchunks_len.checked_add(data_chunk_len), PcmWavError::Overflow];
        let mut offset = unwrap![ok_err_map?
            Riff::write_form_header(dst, Riff::WAVE, subchunks_len), |e|e.to_wav()];
        offset += unwrap! { ok_err_map? Riff::write_chunk_header(slice![mut dst, offset,..],
        Riff::FMT, fmt_len), |e| e.to_wav() };
        offset += unwrap![ok? Self::write_fmt_payload(slice![mut dst, offset,..], fmt)];
        if fmt.needs_fact() {
            offset += unwrap! {ok_err_map? Riff::write_chunk_header(slice![mut dst, offset,..],
            Riff::FACT, 4), |e| e.to_wav() };
            offset += unwrap![ok? write_u32_le(slice![mut dst, offset,..], frames as u32)];
        }
        offset += unwrap! { ok_err_map? Riff::write_chunk_header(slice![mut dst, offset,..],
        Riff::DATA, data.len()), |e| e.to_wav() };
        offset += unwrap![ok? write_bytes(slice![mut dst, offset, ..], data)];
        is! { Riff::pad_len(data.len()) != 0, { dst[offset] = 0; offset += 1; }}
        debug_assert!(offset == len);
        Ok(offset)
    }
    /// Encodes a minimal PCM-family WAVE file into an allocated byte vector.
    #[cfg(feature = "alloc")]
    pub fn to_vec(fmt: PcmWavFmt, data: &[u8]) -> Result<Vec<u8>, PcmWavError> {
        let len = Self::encoded_len(fmt, data.len())?;
        let mut out = vec_![0u8; len];
        let written = Self::encode_into(&mut out, fmt, data)?;
        debug_assert_eq!(written, len);
        Ok(out)
    }
    /// Encodes and writes a minimal PCM-family WAVE file.
    #[cfg(feature = "std")]
    pub fn to_file<P: AsRef<Path>>(
        path: P,
        fmt: PcmWavFmt,
        data: &[u8],
    ) -> Result<(), PcmWavError> {
        let bytes = Self::to_vec(fmt, data)?;
        Fs::write(path, bytes)?;
        Ok(())
    }
    /// Encodes a minimal PCM-family WAVE file from a [`PcmSpec`].
    #[cfg(feature = "alloc")]
    pub fn spec_to_vec(spec: PcmSpec, data: &[u8]) -> Result<Vec<u8>, PcmWavError> {
        let fmt = PcmWavFmt::from_spec(spec)?;
        Self::to_vec(fmt, data)
    }
    /// Writes the WAVE `fmt ` payload.
    const fn write_fmt_payload(dst: &mut [u8], fmt: PcmWavFmt) -> Result<usize, PcmWavError> {
        let len = fmt.encoded_len();
        is! { dst.len() < len, return Err(PcmWavError::NotEnoughSpace) }
        let mut offset = 0;
        offset += unwrap![ok? write_u16_le(slice![mut dst, offset,..], fmt.format_tag)];
        offset += unwrap![ok? write_u16_le(slice![mut dst, offset,..], fmt.channels)];
        offset += unwrap![ok? write_u32_le(slice![mut dst, offset,..], fmt.sample_rate)];
        offset += unwrap![ok? write_u32_le(slice![mut dst, offset,..], fmt.byte_rate)];
        offset += unwrap![ok? write_u16_le(slice![mut dst, offset,..], fmt.block_align)];
        offset += unwrap![ok? write_u16_le(slice![mut dst, offset,..], fmt.bits_per_sample)];
        if len >= PcmWavFmt::WAVEFORMATEX_EMPTY_LEN {
            offset += unwrap![ok? write_u16_le(slice![mut dst, offset,..], fmt.extra_len)];
            // This first writer only supports `cbSize = 0`.
            // Once WAVE_FORMAT_EXTENSIBLE lands, this is where the extra bytes go.
            is! { fmt.extra_len != 0, return Err(PcmWavError::UnsupportedEncoding) }
        }
        Ok(offset)
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
const fn write_bytes(dst: &mut [u8], src: &[u8]) -> Result<usize, PcmWavError> {
    is! { dst.len() < src.len(), return Err(PcmWavError::NotEnoughSpace) }
    whilst! { i in 0..src.len(); { dst[i] = src[i]; }}
    Ok(src.len())
}
const fn write_u16_le(dst: &mut [u8], value: u16) -> Result<usize, PcmWavError> {
    is! { dst.len() < 2, return Err(PcmWavError::NotEnoughSpace) }
    let bytes = value.to_le_bytes();
    dst[0] = bytes[0];
    dst[1] = bytes[1];
    Ok(2)
}
const fn write_u32_le(dst: &mut [u8], value: u32) -> Result<usize, PcmWavError> {
    is! { dst.len() < 4, return Err(PcmWavError::NotEnoughSpace) }
    let bytes = value.to_le_bytes();
    dst[0] = bytes[0];
    dst[1] = bytes[1];
    dst[2] = bytes[2];
    dst[3] = bytes[3];
    Ok(4)
}
