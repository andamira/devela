// devela::media::audio::format::wav::namespace
//
//! Defines [`PcmWav`].
//

#[cfg(feature = "std")]
use crate::{Fs, Path};
#[cfg(feature = "alloc")]
use crate::{PcmSpec, Vec, vec_};
use crate::{
    PcmWavBuf, PcmWavError, PcmWavFmt, Riff, Slice, is, read_at, slice, unwrap, whilst, write_at,
};

#[doc = crate::_tags!(audio codec)]
/// RIFF/WAVE operations for PCM-family audio.
#[doc = crate::_doc_location!("media/audio")]
/// `PcmWav` is a namespace for working with WAVE containers whose payload is
/// raw interleaved PCM-family audio.
///
/// It supports:
/// - parsing borrowed, owned, and file-backed WAVE bytes,
/// - writing WAVE bytes into caller-provided storage,
/// - classic integer PCM and IEEE floating-point WAVE,
/// - WAVE_FORMAT_EXTENSIBLE PCM/float subformats,
/// - conversion from parsed WAVE data to [`PcmRawBuf`].
///
/// The WAVE layer does not reinterpret sample bytes as typed slices. Use the
/// typed helpers on [`PcmRaw`] or [`PcmRawBuf`] to decode or encode samples.
///
/// [`PcmRaw`]: crate::PcmRaw
/// [`PcmRawBuf`]: crate::PcmRawBuf
///
/// # Example
/// ```
/// # use devela::{AudioChannels, PcmRaw, PcmSample, PcmSpec, PcmWav, PcmWavError, PcmWavFmt};
/// # fn main() -> Result<(), PcmWavError> {
/// let spec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 44_100);
/// let fmt = PcmWavFmt::from_spec(spec)?;
///
/// // Two stereo frames: silence, then max-left/min-right.
/// let samples = [0i16, 0, i16::MAX, i16::MIN];
/// let mut pcm = [0u8; 8];
/// PcmRaw::encode_i16_le_into(&mut pcm, spec, &samples)?;
///
/// let len = PcmWav::write_len(fmt, pcm.len())?;
/// let mut wav_bytes = [0u8; 64];
/// let written = PcmWav::write_into(&mut wav_bytes, fmt, &pcm)?;
/// assert_eq!(written, len);
///
/// let wav = PcmWav::parse(&wav_bytes[..written])?;
/// assert_eq!(wav.spec()?, spec);
/// assert_eq!(wav.frames(), 2);
///
/// let raw = wav.raw()?;
/// let mut decoded = [0i16; 4];
/// raw.decode_i16_le_into(&mut decoded)?;
/// assert_eq!(decoded, samples);
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct PcmWav;

/// # Parsing
impl PcmWav {
    /// Parses a borrowed WAVE byte region.
    ///
    /// This validates the RIFF container, finds the first `fmt ` chunk, finds
    /// the first `data` chunk, checks the basic PCM shape, and returns a
    /// byte-backed view into the original input.
    pub const fn parse(bytes: &[u8]) -> Result<PcmWavBuf<&[u8]>, PcmWavError> {
        let parts = unwrap![ok? Self::parse_parts(bytes)];
        Ok(PcmWavBuf::_new(bytes, parts.fmt, parts.data_offset, parts.data_len))
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
        let format_tag = u16::from_le_bytes(read_at![bytes, 0, @2]);
        let extra_len = if bytes.len() >= PcmWavFmt::WAVEFORMATEX_EMPTY_LEN {
            u16::from_le_bytes(read_at![bytes, 16, @2])
        } else {
            0
        };
        let mut fmt = PcmWavFmt {
            format_tag,
            subformat_tag: format_tag,
            channels: u16::from_le_bytes(read_at![bytes, 2, @2]),
            sample_rate: u32::from_le_bytes(read_at![bytes, 4, @4]),
            byte_rate: u32::from_le_bytes(read_at![bytes, 8, @4]),
            block_align: u16::from_le_bytes(read_at![bytes, 12, @2]),
            bits_per_sample: u16::from_le_bytes(read_at![bytes, 14, @2]),
            valid_bits_per_sample: u16::from_le_bytes(read_at![bytes, 14, @2]),
            channel_mask: 0,
            extra_len,
        };
        if format_tag == Self::FORMAT_EXTENSIBLE {
            if bytes.len() < Self::EXTENSIBLE_LEN || extra_len < Self::EXTENSIBLE_EXTRA_LEN {
                return Err(PcmWavError::InvalidExtensibleFormat);
            }
            fmt.valid_bits_per_sample = u16::from_le_bytes(read_at![bytes, 18, @2]);
            fmt.channel_mask = u32::from_le_bytes(read_at![bytes, 20, @4]);
            let guid = read_at![bytes, 24, @16];
            fmt.subformat_tag = match Self::subformat_tag(guid) {
                Ok(tag) => tag,
                Err(err) => return Err(err),
            };
        }
        fmt.validate()
    }

    /// Returns the packed byte width of a supported WAVE sample encoding.
    ///
    /// `format_tag` must be an effective sample format tag, such as
    /// [`FORMAT_PCM`](Self::FORMAT_PCM) or [`FORMAT_IEEE_FLOAT`](Self::FORMAT_IEEE_FLOAT).
    /// For WAVE_FORMAT_EXTENSIBLE, pass the resolved subformat tag instead.
    pub const fn bytes_per_sample(format_tag: u16, bits: u16) -> Result<u16, PcmWavError> {
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
}
/// # Writing
impl PcmWav {
    /// Returns the number of bytes needed to write a minimal WAVE container.
    pub const fn write_len(fmt: PcmWavFmt, data_len: usize) -> Result<usize, PcmWavError> {
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
    /// Writes a minimal WAVE container into `dst`.
    ///
    /// `data` must already contain raw interleaved PCM-family sample bytes matching `fmt`.
    /// This method writes RIFF/WAVE headers and copies the payload;
    /// it does not convert typed samples.
    pub const fn write_into(
        dst: &mut [u8],
        fmt: PcmWavFmt,
        data: &[u8],
    ) -> Result<usize, PcmWavError> {
        use PcmWavError as E;
        let fmt = unwrap![ok? fmt.validate()];
        let frames = unwrap![ok? fmt.frames_for_data_len(data.len())];
        let len = unwrap![ok? Self::write_len(fmt, data.len())];
        is! { dst.len() < len, return Err(E::NotEnoughSpace) }
        is! { frames > u32::MAX as usize, return Err(E::SizeOutOfRange) }
        let fmt_len = fmt.encoded_len();
        let fmt_chunk_len = unwrap![some_ok_or? Riff::chunk_len(fmt_len), E::Overflow];
        let fact_chunk_len = if fmt.needs_fact() {
            unwrap![some_ok_or? Riff::chunk_len(4), E::Overflow]
        } else {
            0
        };
        let data_chunk_len = unwrap![some_ok_or? Riff::chunk_len(data.len()), E::Overflow];
        let subchunks_len = unwrap![some_ok_or?
            fmt_chunk_len.checked_add(fact_chunk_len), E::Overflow];
        let subchunks_len = unwrap![some_ok_or?
            subchunks_len.checked_add(data_chunk_len), E::Overflow];
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
        let len = Self::write_len(fmt, data.len())?;
        let mut out = vec_![0u8; len];
        let written = Self::write_into(&mut out, fmt, data)?;
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
        let mut off = 0;
        off += unwrap![ok? write_u16_le(slice![mut dst, off,..], fmt.format_tag)];
        off += unwrap![ok? write_u16_le(slice![mut dst, off,..], fmt.channels)];
        off += unwrap![ok? write_u32_le(slice![mut dst, off,..], fmt.sample_rate)];
        off += unwrap![ok? write_u32_le(slice![mut dst, off,..], fmt.byte_rate)];
        off += unwrap![ok? write_u16_le(slice![mut dst, off,..], fmt.block_align)];
        off += unwrap![ok? write_u16_le(slice![mut dst, off,..], fmt.bits_per_sample)];
        if fmt.format_tag == PcmWav::FORMAT_EXTENSIBLE {
            is! {
                fmt.extra_len != PcmWav::EXTENSIBLE_EXTRA_LEN,
                return Err(PcmWavError::InvalidExtensibleFormat)
            }
            off += unwrap![ok? write_u16_le(slice![mut dst, off,..], fmt.extra_len)];
            off += unwrap![ok? write_u16_le(slice![mut dst, off,..], fmt.valid_bits_per_sample)];
            off += unwrap![ok? write_u32_le(slice![mut dst, off,..], fmt.channel_mask)];
            let guid = unwrap![ok? PcmWav::subformat_guid(fmt.subformat_tag)];
            write_at![dst, +=off, @16 guid];
            return Ok(off);
        }
        if len >= PcmWavFmt::WAVEFORMATEX_EMPTY_LEN {
            // WAVEFORMATEX extension header. This writer currently supports
            // only an empty extension for non-extensible formats.
            off += unwrap![ok? write_u16_le(slice![mut dst, off,..], fmt.extra_len)];
            is! { fmt.extra_len != 0, return Err(PcmWavError::UnsupportedEncoding) }
        }
        Ok(off)
    }
}
/// # Constants
impl PcmWav {
    /// Integer PCM.
    pub const FORMAT_PCM: u16 = 0x0001;
    /// IEEE floating-point PCM.
    pub const FORMAT_IEEE_FLOAT: u16 = 0x0003;

    /// Extensible WAVE format code.
    pub const FORMAT_EXTENSIBLE: u16 = 0xFFFE;

    /// WAVEFORMATEXTENSIBLE extension byte length.
    pub const EXTENSIBLE_EXTRA_LEN: u16 = 22;

    /// WAVEFORMATEXTENSIBLE `fmt ` payload length.
    pub const EXTENSIBLE_LEN: usize = 40;

    /// WAVEFORMATEXTENSIBLE PCM subformat GUID bytes.
    ///
    /// GUID: `00000001-0000-0010-8000-00AA00389B71`
    pub const SUBFORMAT_PCM: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B,
        0x71,
    ];

    /// WAVEFORMATEXTENSIBLE IEEE float subformat GUID bytes.
    ///
    /// GUID: `00000003-0000-0010-8000-00AA00389B71`
    pub const SUBFORMAT_IEEE_FLOAT: [u8; 16] = [
        0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B,
        0x71,
    ];

    /// Returns the WAVE format tag represented by a supported extensible subformat GUID.
    pub const fn subformat_tag(guid: [u8; 16]) -> Result<u16, PcmWavError> {
        if Slice::<u8>::eq(&guid, &Self::SUBFORMAT_PCM) {
            Ok(Self::FORMAT_PCM)
        } else if Slice::<u8>::eq(&guid, &Self::SUBFORMAT_IEEE_FLOAT) {
            Ok(Self::FORMAT_IEEE_FLOAT)
        } else {
            Err(PcmWavError::UnsupportedSubformat)
        }
    }
    /// Returns the WAVEFORMATEXTENSIBLE GUID for a supported sample format tag.
    pub const fn subformat_guid(format_tag: u16) -> Result<[u8; 16], PcmWavError> {
        match format_tag {
            Self::FORMAT_PCM => Ok(Self::SUBFORMAT_PCM),
            Self::FORMAT_IEEE_FLOAT => Ok(Self::SUBFORMAT_IEEE_FLOAT),
            _ => Err(PcmWavError::UnsupportedSubformat),
        }
    }
}

#[derive(Clone, Copy)]
struct PcmWavParts {
    fmt: PcmWavFmt,
    data_offset: usize,
    data_len: usize,
}
const fn write_bytes(dst: &mut [u8], src: &[u8]) -> Result<usize, PcmWavError> {
    is! { dst.len() < src.len(), return Err(PcmWavError::NotEnoughSpace) }
    whilst! { i in 0..src.len(); { dst[i] = src[i]; }}
    Ok(src.len())
}
const fn write_u16_le(dst: &mut [u8], value: u16) -> Result<usize, PcmWavError> {
    is! { dst.len() < 2, return Err(PcmWavError::NotEnoughSpace) }
    write_at![dst, 0, @2 value.to_le_bytes()];
    Ok(2)
}
const fn write_u32_le(dst: &mut [u8], value: u32) -> Result<usize, PcmWavError> {
    is! { dst.len() < 4, return Err(PcmWavError::NotEnoughSpace) }
    write_at![dst, 0, @4 value.to_le_bytes()];
    Ok(4)
}
