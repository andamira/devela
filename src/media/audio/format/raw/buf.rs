// devela::media::audio::format::raw::buf
//
//! Defines [`PcmRawBuf`].
//

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{AudioChannels, PcmRaw, PcmRawError, PcmSample, PcmSpec};

#[doc = crate::_tags!(audio data)]
/// Raw PCM byte buffer over borrowed or owned storage.
#[doc = crate::_doc_meta!{
    location("media/audio"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(PcmRawBuf_Slice_x32: PcmRawBuf<&[u8]> = 16|128),
    #[cfg(target_pointer_width = "64")]
    test_size_of(PcmRawBuf_Slice_x64: PcmRawBuf<&[u8]> = 24|192),
    #[cfg(target_pointer_width = "32")]
    test_size_of(PcmRawBuf_Vec_x32: PcmRawBuf<Vec<u8>> = 20|160),
    #[cfg(target_pointer_width = "64")]
    test_size_of(PcmRawBuf_Vec_x64: PcmRawBuf<Vec<u8>> = 32|256),
}]
///
/// Raw PCM contains no header and no embedded metadata. The caller must provide
/// the [`PcmSpec`] that describes the byte stream.
///
/// This is a validated pairing of raw interleaved PCM bytes and a `PcmSpec`.
/// It does not own typed samples. Typed materialization is explicit through
/// [`PcmRaw`] decode helpers.
///
/// The storage type decides ownership:
/// - `PcmRawBuf<&[u8]>` borrows existing raw PCM bytes.
/// - `PcmRawBuf<Vec<u8>>` owns allocated raw PCM bytes.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PcmRawBuf<B> {
    /// Raw interleaved PCM bytes.
    bytes: B,
    /// Caller-provided stream metadata.
    spec: PcmSpec,
}
#[rustfmt::skip]
impl<B> PcmRawBuf<B> {
    /// Creates a raw PCM buffer from already-validated parts.
    pub(crate) const fn _new(bytes: B, spec: PcmSpec) -> Self { Self { bytes, spec } }
    /// Returns the stream metadata.
    pub const fn spec(&self) -> PcmSpec { self.spec }
    /// Returns the encoded sample format.
    pub const fn sample(&self) -> PcmSample { self.spec.sample }
    /// Returns the channel layout.
    #[must_use]
    pub const fn channels(&self) -> AudioChannels { self.spec.channels }
    /// Returns the sample rate in Hertz.
    #[must_use]
    pub const fn sample_rate(&self) -> u32 { self.spec.sample_rate }
    /// Returns the byte size of one interleaved frame.
    #[must_use]
    pub const fn frame_bytes(&self) -> usize { self.spec.frame_bytes() }
}
#[rustfmt::skip]
impl<B: AsRef<[u8]>> PcmRawBuf<B> {
    /// Returns the raw interleaved PCM bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] { self.bytes.as_ref() }
    /// Returns whether the byte stream is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.bytes.as_ref().is_empty() }
    /// Returns the byte length of the raw stream.
    #[must_use]
    pub fn len(&self) -> usize { self.bytes.as_ref().len() }
    /// Returns the number of complete interleaved frames.
    pub fn frames(&self) -> Result<usize, PcmRawError> {
        self.spec.frames_for_data_len(self.len()).ok_or(PcmRawError::InvalidDataLength)
    }
    /// Returns whether the byte stream contains complete interleaved frames.
    #[must_use]
    pub fn has_complete_frames(&self) -> bool {
        self.spec.has_complete_frames_for_data_len(self.len())
    }

    /* decode helpers */

    /// Decodes unsigned 8-bit PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub fn decode_u8_into(&self, dst: &mut [u8]) -> Result<usize, PcmRawError> {
        PcmRaw::decode_u8_into(self.bytes(), self.spec(), dst)
    }
    /// Decodes signed 8-bit PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub fn decode_i8_into(&self, dst: &mut [i8]) -> Result<usize, PcmRawError> {
        PcmRaw::decode_i8_into(self.bytes(), self.spec(), dst)
    }
    /// Decodes little-endian signed 16-bit PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub fn decode_i16_le_into(&self, dst: &mut [i16]) -> Result<usize, PcmRawError> {
        PcmRaw::decode_i16_le_into(self.bytes(), self.spec(), dst)
    }
    /// Decodes little-endian signed 24-bit PCM samples into `dst`.
    ///
    /// Samples are sign-extended into `i32`.
    ///
    /// Returns the number of samples written.
    pub fn decode_i24_le_into(&self, dst: &mut [i32]) -> Result<usize, PcmRawError> {
        PcmRaw::decode_i24_le_into(self.bytes(), self.spec(), dst)
    }
    /// Decodes little-endian signed 32-bit PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub fn decode_i32_le_into(&self, dst: &mut [i32]) -> Result<usize, PcmRawError> {
        PcmRaw::decode_i32_le_into(self.bytes(), self.spec(), dst)
    }
    /// Decodes little-endian 32-bit floating-point PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub fn decode_f32_le_into(&self, dst: &mut [f32]) -> Result<usize, PcmRawError> {
        PcmRaw::decode_f32_le_into(self.bytes(), self.spec(), dst)
    }
    /// Decodes little-endian 64-bit floating-point PCM samples into `dst`.
    ///
    /// Returns the number of samples written.
    pub fn decode_f64_le_into(&self, dst: &mut [f64]) -> Result<usize, PcmRawError> {
        PcmRaw::decode_f64_le_into(self.bytes(), self.spec(), dst)
    }
}
#[rustfmt::skip]
impl<'a> PcmRawBuf<&'a [u8]> {
    /// Returns the raw interleaved PCM bytes.
    ///
    /// Const-friendly alternative to [`bytes`](Self::bytes).
    #[must_use]
    pub const fn bytes_const(&self) -> &'a [u8] { self.bytes }
    /// Returns whether the byte stream is empty.
    #[must_use]
    pub const fn is_empty_const(&self) -> bool { self.bytes.is_empty() }
    /// Returns the byte length of the raw stream.
    #[must_use]
    pub const fn len_const(&self) -> usize { self.bytes.len() }
    /// Returns the number of complete interleaved frames.
    pub const fn frames_const(&self) -> Result<usize, PcmRawError> {
        match self.spec.frames_for_data_len(self.bytes.len()) {
            Some(frames) => Ok(frames),
            None => Err(PcmRawError::InvalidDataLength),
        }
    }
}
#[cfg(feature = "alloc")]
impl PcmRawBuf<Vec<u8>> {
    /// Returns this owned raw PCM buffer as a borrowed raw PCM buffer.
    pub fn as_borrowed(&self) -> PcmRawBuf<&[u8]> {
        PcmRawBuf::_new(self.bytes.as_slice(), self.spec)
    }
    /// Returns the owned raw PCM bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}
