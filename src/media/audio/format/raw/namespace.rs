// devela::media::audio::format::raw::namespace
//
//! Defines [`PcmRaw`].
//

#[cfg(feature = "alloc")]
use crate::Vec;
#[cfg(feature = "std")]
use crate::{Fs, Path};
use crate::{PcmRawBuf, PcmRawError, PcmSample, PcmSpec, is, whilst};

#[doc = crate::_tags!(audio)]
/// Headerless raw PCM operations.
#[doc = crate::_doc_location!("media/audio")]
#[derive(Debug)]
pub struct PcmRaw;
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
    /// Alias for parser-style use from format modules.
    pub const fn decode(bytes: &[u8], spec: PcmSpec) -> Result<PcmRawBuf<&[u8]>, PcmRawError> {
        Self::from_bytes(bytes, spec)
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
}
impl PcmRaw {
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
        let channels = spec.channel_count() as usize;
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
}

// Decode bodies
impl PcmRaw {
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
                let mut o = 0;
                whilst! { i in 0..samples; {
                    dst[i] = i16::from_le_bytes([bytes[o], bytes[o + 1]]);
                    o += 2;
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
                let mut o = 0;
                whilst! { i in 0..samples; {
                    dst[i] = i32::from_le_bytes([
                        bytes[o],
                        bytes[o + 1],
                        bytes[o + 2],
                        bytes[o + 3],
                    ]);
                    o += 4;
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
                let mut o = 0;
                whilst! { i in 0..samples; {
                    let bits = u32::from_le_bytes([
                        bytes[o],
                        bytes[o + 1],
                        bytes[o + 2],
                        bytes[o + 3],
                    ]);
                    dst[i] = f32::from_bits(bits);
                    o += 4;
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
                let mut o = 0;
                whilst! { i in 0..samples; {
                    let bits = u64::from_le_bytes([
                        bytes[o],
                        bytes[o + 1],
                        bytes[o + 2],
                        bytes[o + 3],
                        bytes[o + 4],
                        bytes[o + 5],
                        bytes[o + 6],
                        bytes[o + 7],
                    ]);
                    dst[i] = f64::from_bits(bits);
                    o += 8;
                }}
                Ok(samples)
            }
            Err(err) => Err(err),
        }
    }
}
// Encode bodies
impl PcmRaw {
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
                let mut o = 0;
                whilst! { i in 0..samples.len(); {
                    let b = samples[i].to_le_bytes();
                    dst[o] = b[0];
                    dst[o + 1] = b[1];
                    o += 2;
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
                let mut o = 0;
                whilst! { i in 0..samples.len(); {
                    let b = samples[i].to_le_bytes();
                    dst[o] = b[0];
                    dst[o + 1] = b[1];
                    dst[o + 2] = b[2];
                    dst[o + 3] = b[3];
                    o += 4;
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
                let mut o = 0;
                whilst! { i in 0..samples.len(); {
                    let b = samples[i].to_bits().to_le_bytes();
                    dst[o] = b[0];
                    dst[o + 1] = b[1];
                    dst[o + 2] = b[2];
                    dst[o + 3] = b[3];
                    o += 4;
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
                let mut o = 0;
                whilst! { i in 0..samples.len(); {
                    let b = samples[i].to_bits().to_le_bytes();
                    dst[o] = b[0];
                    dst[o + 1] = b[1];
                    dst[o + 2] = b[2];
                    dst[o + 3] = b[3];
                    dst[o + 4] = b[4];
                    dst[o + 5] = b[5];
                    dst[o + 6] = b[6];
                    dst[o + 7] = b[7];
                    o += 8;
                }}
                Ok(bytes_len)
            }
            Err(err) => Err(err),
        }
    }
}
