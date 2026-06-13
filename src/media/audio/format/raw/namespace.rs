// devela/src/media/audio/format/raw/namespace.rs
//
//! Defines [`PcmRaw`].
//

#[cfg(feature = "alloc")]
use crate::Vec;
#[cfg(feature = "std")]
use crate::{Fs, Path};
use crate::{PcmRawBuf, PcmRawError, PcmSample, PcmSpec, is, read_at, whilst, write_at};

#[doc = crate::_tags!(audio codec)]
/// Headerless raw PCM operations.
#[doc = crate::_doc_meta!{location("media/audio")}]
///
/// Raw PCM has no embedded metadata. Callers must provide a [`PcmSpec`]
/// describing the byte stream.
///
/// This namespace separates three operations:
/// - [`from_bytes`](Self::from_bytes) binds raw bytes to a caller-provided spec.
/// - [`write_into`](Self::write_into) copies already-encoded raw PCM bytes.
/// - `decode_*` / `encode_*` convert between raw PCM bytes and typed samples.
///
/// # Examples
/// ```rust
/// use devela::{AudioChannels, PcmRaw, PcmSample, PcmSpec};
///
/// let spec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 44_100);
/// let bytes = b"\0\0\0\0\xFF\x7F\0\x80";
///
/// let raw = PcmRaw::from_bytes(bytes, spec)?;
/// assert_eq!(raw.frames()?, 2);
///
/// let mut samples = [0i16; 4];
/// let count = raw.decode_i16_le_into(&mut samples)?;
///
/// assert_eq!(count, 4);
/// assert_eq!(samples, [0, 0, 32767, -32768]);
///
/// let mut encoded = [0u8; 8];
/// let written = PcmRaw::encode_i16_le_into(&mut encoded, spec, &samples)?;
///
/// assert_eq!(written, bytes.len());
/// assert_eq!(&encoded, bytes);
///
/// # Ok::<(), devela::PcmRawError>(())
/// ```
#[derive(Debug)]
pub struct PcmRaw;

/// # Parsing
impl PcmRaw {
    /// Creates a borrowed raw PCM byte buffer from caller-provided metadata.
    ///
    /// Raw PCM has no header, so the stream metadata must be supplied by the caller.
    pub const fn from_bytes(bytes: &[u8], spec: PcmSpec) -> Result<PcmRawBuf<&[u8]>, PcmRawError> {
        is! { !spec.is_valid(), return Err(PcmRawError::InvalidSpec) }
        if spec.frames_for_data_len(bytes.len()).is_none() {
            return Err(PcmRawError::InvalidDataLength);
        }
        Ok(PcmRawBuf::_new(bytes, spec))
    }
    /// Creates an owned raw PCM byte buffer from caller-provided metadata.
    #[cfg(feature = "alloc")]
    pub fn from_vec(bytes: Vec<u8>, spec: PcmSpec) -> Result<PcmRawBuf<Vec<u8>>, PcmRawError> {
        let _ = Self::from_bytes(bytes.as_slice(), spec)?; // just for validation
        Ok(PcmRawBuf::_new(bytes, spec))
    }
    /// Reads a raw PCM file using caller-provided metadata.
    #[cfg(feature = "std")]
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        spec: PcmSpec,
    ) -> Result<PcmRawBuf<Vec<u8>>, PcmRawError> {
        Self::from_vec(Fs::read(path)?, spec)
    }
    /* typed decoding */

    /// Checks a typed decode operation and returns the sample count.
    const fn check_decode(
        bytes: &[u8],
        spec: PcmSpec,
        expected: PcmSample,
        sample_bytes: usize,
        dst_samples: usize,
    ) -> Result<usize, PcmRawError> {
        use PcmRawError as E;
        is! { !spec.is_valid(), return Err(E::InvalidSpec) }
        is! { !spec.sample.eq(expected), return Err(E::MismatchedSampleFormat) }
        is! { !bytes.len().is_multiple_of(sample_bytes), return Err(E::InvalidDataLength) }
        is! { spec.frames_for_data_len(bytes.len()).is_none(), return Err(E::InvalidDataLength) }
        let samples = bytes.len() / sample_bytes;
        is! { dst_samples < samples, return Err(E::NotEnoughSpace) }
        Ok(samples)
    }
    /// Decodes unsigned 8-bit PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub const fn decode_u8_into(
        bytes: &[u8],
        spec: PcmSpec,
        dst: &mut [u8],
    ) -> Result<usize, PcmRawError> {
        match Self::check_decode(bytes, spec, PcmSample::U8, 1, dst.len()) {
            Ok(samples) => {
                whilst! { i in 0..samples; { dst[i] = bytes[i]; }}
                Ok(samples)
            }
            Err(err) => Err(err),
        }
    }
    /// Decodes signed 8-bit PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub const fn decode_i8_into(
        bytes: &[u8],
        spec: PcmSpec,
        dst: &mut [i8],
    ) -> Result<usize, PcmRawError> {
        match Self::check_decode(bytes, spec, PcmSample::I8, 1, dst.len()) {
            Ok(samples) => {
                whilst! { i in 0..samples; { dst[i] = bytes[i] as i8; }}
                Ok(samples)
            }
            Err(err) => Err(err),
        }
    }
    /// Decodes little-endian signed 16-bit PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub const fn decode_i16_le_into(
        bytes: &[u8],
        spec: PcmSpec,
        dst: &mut [i16],
    ) -> Result<usize, PcmRawError> {
        match Self::check_decode(bytes, spec, PcmSample::I16, 2, dst.len()) {
            Ok(samples) => {
                let mut offset = 0;
                whilst! { i in 0..samples; {
                    dst[i] = i16::from_le_bytes(read_at![bytes, +=offset, @2]);
                }}
                Ok(samples)
            }
            Err(err) => Err(err),
        }
    }
    /// Decodes little-endian signed 24-bit PCM samples into `dst`.
    ///
    /// Samples are sign-extended into `i32`.
    ///
    /// Returns the number of samples written.
    pub const fn decode_i24_le_into(
        bytes: &[u8],
        spec: PcmSpec,
        dst: &mut [i32],
    ) -> Result<usize, PcmRawError> {
        match Self::check_decode(bytes, spec, PcmSample::I24, 3, dst.len()) {
            Ok(samples) => {
                let mut offset = 0;
                whilst! { i in 0..samples; {
                    let b = read_at![bytes, +=offset, @3];
                    let unsigned = (b[0] as i32) | ((b[1] as i32) << 8) | ((b[2] as i32) << 16);
                    // sign-extend bit 23 into bits 24..31
                    dst[i] = is![unsigned & 0x0080_0000 != 0, unsigned | !0x00FF_FFFF, unsigned];
                }}
                Ok(samples)
            }
            Err(err) => Err(err),
        }
    }
    /// Decodes little-endian signed 32-bit PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub const fn decode_i32_le_into(
        bytes: &[u8],
        spec: PcmSpec,
        dst: &mut [i32],
    ) -> Result<usize, PcmRawError> {
        match Self::check_decode(bytes, spec, PcmSample::I32, 4, dst.len()) {
            Ok(samples) => {
                let mut offset = 0;
                whilst! { i in 0..samples; {
                    dst[i] = i32::from_le_bytes(read_at![bytes, +=offset, @4]);
                }}
                Ok(samples)
            }
            Err(err) => Err(err),
        }
    }
    /// Decodes little-endian 32-bit floating-point PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub const fn decode_f32_le_into(
        bytes: &[u8],
        spec: PcmSpec,
        dst: &mut [f32],
    ) -> Result<usize, PcmRawError> {
        match Self::check_decode(bytes, spec, PcmSample::F32, 4, dst.len()) {
            Ok(samples) => {
                let mut offset = 0;
                whilst! { i in 0..samples; {
                    dst[i] = f32::from_bits(u32::from_le_bytes(read_at![bytes, +=offset, @4]));
                }}
                Ok(samples)
            }
            Err(err) => Err(err),
        }
    }
    /// Decodes little-endian 64-bit floating-point PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub const fn decode_f64_le_into(
        bytes: &[u8],
        spec: PcmSpec,
        dst: &mut [f64],
    ) -> Result<usize, PcmRawError> {
        match Self::check_decode(bytes, spec, PcmSample::F64, 8, dst.len()) {
            Ok(samples) => {
                let mut offset = 0;
                whilst! { i in 0..samples; {
                    dst[i] = f64::from_bits(u64::from_le_bytes(read_at![bytes, +=offset, @8]));
                }}
                Ok(samples)
            }
            Err(err) => Err(err),
        }
    }
}
/// # Writing
impl PcmRaw {
    /// Returns the encoded raw PCM byte length.
    ///
    /// For raw PCM this is exactly the payload length, after validating that it
    /// contains complete frames for `spec`.
    pub const fn written_len(spec: PcmSpec, data_len: usize) -> Result<usize, PcmRawError> {
        is! { !spec.is_valid(), return Err(PcmRawError::InvalidSpec) }
        if spec.frames_for_data_len(data_len).is_none() {
            return Err(PcmRawError::InvalidDataLength);
        }
        Ok(data_len)
    }
    /// Writes raw PCM bytes into `dst`.
    ///
    /// This does not transform endianness or sample representation. It copies the
    /// given raw byte stream as-is after validating its frame shape.
    pub const fn write_into(
        dst: &mut [u8],
        spec: PcmSpec,
        data: &[u8],
    ) -> Result<usize, PcmRawError> {
        let len = match Self::written_len(spec, data.len()) {
            Ok(len) => len,
            Err(err) => return Err(err),
        };
        is! { dst.len() < len, return Err(PcmRawError::NotEnoughSpace) }
        whilst! { i in 0..data.len(); { dst[i] = data[i]; }}
        Ok(len)
    }
    /// Copies raw PCM bytes into an allocated vector.
    #[cfg(feature = "alloc")]
    pub fn to_vec(spec: PcmSpec, data: &[u8]) -> Result<Vec<u8>, PcmRawError> {
        Self::written_len(spec, data.len())?;
        Ok(data.to_vec())
    }
    /// Writes raw PCM bytes to a file.
    #[cfg(feature = "std")]
    pub fn to_file<P: AsRef<Path>>(path: P, spec: PcmSpec, data: &[u8]) -> Result<(), PcmRawError> {
        Self::written_len(spec, data.len())?;
        Fs::write(path, data)?;
        Ok(())
    }
    /* typed encoding */

    /// Checks a typed encode operation and returns the byte count.
    const fn check_encode(
        dst_len: usize,
        spec: PcmSpec,
        expected: PcmSample,
        sample_bytes: usize,
        sample_len: usize,
    ) -> Result<usize, PcmRawError> {
        use PcmRawError as E;
        is! { !spec.is_valid(), return Err(E::InvalidSpec) }
        is! { !spec.sample.eq(expected), return Err(E::MismatchedSampleFormat) }
        let channels = spec.channel_count();
        if channels == 0 || !sample_len.is_multiple_of(channels) {
            return Err(E::InvalidDataLength);
        }
        let Some(bytes_len) = sample_len.checked_mul(sample_bytes) else {
            return Err(E::InvalidDataLength);
        };
        is! { spec.frames_for_data_len(bytes_len).is_none(), return Err(E::InvalidDataLength) }
        is! { dst_len < bytes_len,return Err(E::NotEnoughSpace) }
        Ok(bytes_len)
    }
    /// Encodes unsigned 8-bit PCM samples into `dst`.
    ///
    /// Returns the number of bytes written.
    pub const fn encode_u8_into(
        dst: &mut [u8],
        spec: PcmSpec,
        samples: &[u8],
    ) -> Result<usize, PcmRawError> {
        match Self::check_encode(dst.len(), spec, PcmSample::U8, 1, samples.len()) {
            Ok(bytes_len) => {
                whilst! { i in 0..samples.len(); { dst[i] = samples[i]; }}
                Ok(bytes_len)
            }
            Err(err) => Err(err),
        }
    }
    /// Encodes signed 8-bit PCM samples into `dst`.
    ///
    /// Returns the number of bytes written.
    pub const fn encode_i8_into(
        dst: &mut [u8],
        spec: PcmSpec,
        samples: &[i8],
    ) -> Result<usize, PcmRawError> {
        match Self::check_encode(dst.len(), spec, PcmSample::I8, 1, samples.len()) {
            Ok(bytes_len) => {
                whilst! { i in 0..samples.len(); { dst[i] = samples[i] as u8; }}
                Ok(bytes_len)
            }
            Err(err) => Err(err),
        }
    }
    /// Encodes signed 16-bit PCM samples as little-endian bytes into `dst`.
    ///
    /// Returns the number of bytes written.
    pub const fn encode_i16_le_into(
        dst: &mut [u8],
        spec: PcmSpec,
        samples: &[i16],
    ) -> Result<usize, PcmRawError> {
        match Self::check_encode(dst.len(), spec, PcmSample::I16, 2, samples.len()) {
            Ok(bytes_len) => {
                let mut offset = 0;
                whilst! { i in 0..samples.len(); {
                    write_at![dst, +=offset, @2 samples[i].to_le_bytes()];
                }}
                Ok(bytes_len)
            }
            Err(err) => Err(err),
        }
    }
    /// Encodes signed 24-bit PCM samples as little-endian bytes into `dst`.
    ///
    /// Input samples must fit the signed 24-bit range:
    /// `-8_388_608..=8_388_607`.
    ///
    /// Returns the number of bytes written.
    pub const fn encode_i24_le_into(
        dst: &mut [u8],
        spec: PcmSpec,
        samples: &[i32],
    ) -> Result<usize, PcmRawError> {
        match Self::check_encode(dst.len(), spec, PcmSample::I24, 3, samples.len()) {
            Ok(bytes_len) => {
                let mut offset = 0;
                whilst! { i in 0..samples.len(); {
                    let sample = samples[i];
                    if sample < -8_388_608 || sample > 8_388_607 {
                        return Err(PcmRawError::SampleOutOfRange);
                    }
                    // low 24 bits, two's-complement little-endian
                    write_at![dst, +=offset, @3 sample.to_le_bytes()];
                }}
                Ok(bytes_len)
            }
            Err(err) => Err(err),
        }
    }
    /// Encodes signed 32-bit PCM samples as little-endian bytes into `dst`.
    ///
    /// Returns the number of bytes written.
    pub const fn encode_i32_le_into(
        dst: &mut [u8],
        spec: PcmSpec,
        samples: &[i32],
    ) -> Result<usize, PcmRawError> {
        match Self::check_encode(dst.len(), spec, PcmSample::I32, 4, samples.len()) {
            Ok(bytes_len) => {
                let mut offset = 0;
                whilst! { i in 0..samples.len(); {
                    write_at![dst, +=offset, @4 samples[i].to_le_bytes()];
                }}
                Ok(bytes_len)
            }
            Err(err) => Err(err),
        }
    }
    /// Encodes 32-bit floating-point PCM samples as little-endian bytes into `dst`.
    ///
    /// Returns the number of bytes written.
    pub const fn encode_f32_le_into(
        dst: &mut [u8],
        spec: PcmSpec,
        samples: &[f32],
    ) -> Result<usize, PcmRawError> {
        match Self::check_encode(dst.len(), spec, PcmSample::F32, 4, samples.len()) {
            Ok(bytes_len) => {
                let mut offset = 0;
                whilst! { i in 0..samples.len(); {
                    write_at![dst, +=offset, @4 samples[i].to_bits().to_le_bytes()];
                }}
                Ok(bytes_len)
            }
            Err(err) => Err(err),
        }
    }
    /// Encodes 64-bit floating-point PCM samples as little-endian bytes into `dst`.
    ///
    /// Returns the number of bytes written.
    pub const fn encode_f64_le_into(
        dst: &mut [u8],
        spec: PcmSpec,
        samples: &[f64],
    ) -> Result<usize, PcmRawError> {
        match Self::check_encode(dst.len(), spec, PcmSample::F64, 8, samples.len()) {
            Ok(bytes_len) => {
                let mut offset = 0;
                whilst! { i in 0..samples.len(); {
                    write_at![dst, +=offset, @8 samples[i].to_bits().to_le_bytes()];
                }}
                Ok(bytes_len)
            }
            Err(err) => Err(err),
        }
    }
}
