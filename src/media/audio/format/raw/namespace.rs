// devela::media::audio::format::raw::namespace
//
//! Defines [`PcmRaw`].
//

#[cfg(feature = "alloc")]
use crate::Vec;
#[cfg(feature = "std")]
use crate::{Fs, Path};
use crate::{PcmRawBuf, PcmRawError, PcmSpec, is, whilst};

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
    pub const fn encode_len(spec: PcmSpec, data_len: usize) -> Result<usize, PcmRawError> {
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
    pub const fn encode_into(
        dst: &mut [u8],
        spec: PcmSpec,
        data: &[u8],
    ) -> Result<usize, PcmRawError> {
        let len = match Self::encode_len(spec, data.len()) {
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
        Self::encode_len(spec, data.len())?;
        Ok(data.to_vec())
    }
    /// Writes raw PCM bytes to a file.
    #[cfg(feature = "std")]
    pub fn to_file<P: AsRef<Path>>(path: P, spec: PcmSpec, data: &[u8]) -> Result<(), PcmRawError> {
        Self::encode_len(spec, data.len())?;
        Fs::write(path, data)?;
        Ok(())
    }
}
