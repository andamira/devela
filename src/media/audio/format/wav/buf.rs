// devela::media::audio::format::wav::buf
//
//! Defines [`PcmWavBuf`].
//

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{PcmRawBuf, PcmSpec, PcmWavError, PcmWavFmt, Riff, RiffChunkIter};

crate::test_size_of![PcmWavBuf_Slice: PcmWavBuf<&[u8]> = 56]; // 448 bits
#[cfg(feature = "alloc")]
crate::test_size_of![PcmWavBuf_Vec: PcmWavBuf<Vec<u8>> = 64]; // 512 bits

#[doc = crate::_tags!(audio data)]
/// Parsed WAVE byte buffer over borrowed or owned storage.
#[doc = crate::_doc_location!("media/audio")]
///
/// This stores the full WAVE byte region together with the parsed `fmt ` metadata
/// and the byte span of the `data` payload.
///
/// The storage type decides ownership:
/// - `PcmWavBuf<&[u8]>` borrows existing WAVE bytes.
/// - `PcmWavBuf<Vec<u8>>` owns allocated WAVE bytes.
///
/// The audio payload is still raw bytes. Typed PCM decoding should be explicit,
/// because WAVE integer samples are little-endian and may not be naturally aligned.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PcmWavBuf<B> {
    /// Full WAVE byte region.
    bytes: B,
    /// Parsed `fmt ` chunk.
    fmt: PcmWavFmt,
    /// Byte offset of the `data` payload within `bytes`.
    data_offset: usize,
    /// Byte length of the `data` payload.
    data_len: usize,
}
#[rustfmt::skip]
impl<B> PcmWavBuf<B> {
    /// Creates a parsed WAVE buffer from already-validated parts.
    ///
    /// This is crate-private because the offset and length must remain valid
    /// for `bytes`, and the `fmt ` metadata must have already been checked.
    pub(crate) const fn _new(bytes: B, fmt: PcmWavFmt, data_offset: usize, data_len: usize)
        -> Self { Self { bytes, fmt, data_offset, data_len } }

    /// Returns the parsed `fmt ` metadata.
    pub const fn fmt(&self) -> PcmWavFmt { self.fmt }
    /// Returns the byte offset of the `data` payload in the full WAVE byte region.
    #[must_use]
    pub const fn data_offset(&self) -> usize { self.data_offset }
    /// Returns the byte length of the `data` payload.
    #[must_use]
    pub const fn data_len(&self) -> usize { self.data_len }
    /// Returns the byte offset and length of the `data` payload.
    #[must_use]
    pub const fn data_span(&self) -> (usize, usize) { (self.data_offset, self.data_len) }
    /// Returns the number of interleaved PCM frames.
    #[must_use]
    pub const fn frames(&self) -> usize { self.data_len / self.fmt.block_align as usize }
    /// Maps this WAVE metadata to current [`PcmSpec`] metadata when unambiguous.
    pub const fn spec(&self) -> Result<PcmSpec, PcmWavError> { self.fmt.spec() }
}
#[rustfmt::skip]
impl<B: AsRef<[u8]>> PcmWavBuf<B> {
    /// Returns the full WAVE byte region.
    #[must_use]
    pub fn bytes(&self) -> &[u8] { self.bytes.as_ref() }

    /// Returns the borrowed `data` chunk payload.
    ///
    /// This slices the original WAVE byte region using the span found during
    /// parsing. The fields are private so safe constructors preserve the span.
    #[must_use]
    pub fn data_bytes(&self) -> &[u8] {
        let start = self.data_offset;
        let end = start + self.data_len;
        &self.bytes.as_ref()[start..end]
    }

    /// Returns whether the `data` chunk payload is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.data_len == 0 }

    /// Returns an iterator over the WAVE subchunks.
    pub fn chunks(&self) -> Result<RiffChunkIter<'_>, PcmWavError> {
        let root = Riff::root(self.bytes()).map_err(PcmWavError::Riff)?;
        root.subchunks().map_err(PcmWavError::Riff)
    }

    /// Returns the WAVE `data` chunk as headerless raw PCM.
    pub fn raw(&self) -> Result<PcmRawBuf<&[u8]>, PcmWavError> {
        let spec = self.spec()?;
        Ok(PcmRawBuf::_new(self.data_bytes(), spec))
    }
}
#[rustfmt::skip]
impl<'a> PcmWavBuf<&'a [u8]> {
    /// Returns the full borrowed WAVE byte region.
    ///
    /// This exists as a const-friendly alternative to [`bytes`](Self::bytes).
    #[must_use]
    pub const fn bytes_const(&self) -> &'a [u8] { self.bytes }
}
#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl PcmWavBuf<Vec<u8>> {
    /// Returns this owned WAVE buffer as a borrowed WAVE buffer.
    pub fn as_borrowed(&self) -> PcmWavBuf<&[u8]> {
        PcmWavBuf::_new(self.bytes.as_slice(), self.fmt, self.data_offset, self.data_len)
    }
    /// Returns the owned full WAVE byte region.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> { self.bytes }
}
