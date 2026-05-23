// devela::data::codec::pack::wrap::riff::namespace
//
//! Defines [`Riff`].
//

use crate::{BinTag4, RiffChunk, RiffChunkIter, RiffError, is, slice, unwrap};

#[doc = crate::_tags!(data codec)]
/// RIFF tagged binary container operations.
#[doc = crate::_doc_location!("data/codec/pack")]
///
/// RIFF stores data as a sequence of tagged chunks.
/// Each chunk has a four-byte identifier, a little-endian payload size,
/// and a payload followed by one optional pad byte when the payload size is odd.
///
/// This namespace provides borrowed, allocation-free parsing over RIFF-like
/// byte regions. Unknown chunks are preserved as raw byte slices and can be
/// skipped by their declared size.
///
/// # Example
/// ```
/// # use devela::{BinTag4, Riff};
/// let bytes = b"RIFF\x10\0\0\0WAVEfmt \x04\0\0\0\x01\0\x02\0";
///
/// let root = Riff::root(bytes)?;
/// assert_eq![root.id(), Riff::RIFF];
/// assert_eq![root.container_type(), Some(Riff::WAVE)];
///
/// let mut chunks = root.subchunks()?;
/// let fmt = chunks.next_chunk().unwrap()?;
/// assert_eq![fmt.id(), BinTag4::new(*b"fmt ")];
/// assert_eq![fmt.data(), &[1, 0, 2, 0]];
/// assert![chunks.next_chunk().is_none()];
/// # Ok::<(), devela::RiffError>(())
/// ```
#[derive(Debug)]
pub struct Riff;

impl Riff {
    /// RIFF little-endian root chunk.
    pub const RIFF: BinTag4 = BinTag4::new(*b"RIFF");
    /// RIFF big-endian root chunk.
    pub const RIFX: BinTag4 = BinTag4::new(*b"RIFX");
    /// RIFF list chunk.
    pub const LIST: BinTag4 = BinTag4::new(*b"LIST");
    /// Common WAVE form type.
    pub const WAVE: BinTag4 = BinTag4::new(*b"WAVE");
    /// Common WAVE format chunk.
    pub const FMT: BinTag4 = BinTag4::new(*b"fmt ");
    /// Common binary data chunk.
    pub const DATA: BinTag4 = BinTag4::new(*b"data");
    /// Common filler chunk.
    pub const JUNK: BinTag4 = BinTag4::new(*b"JUNK");
    /// Common associated-info list type.
    pub const INFO: BinTag4 = BinTag4::new(*b"INFO");

    /// Returns the RIFF pad length for a chunk data length.
    #[must_use]
    #[inline(always)]
    pub const fn pad_len(len: usize) -> usize {
        len & 1
    }
    /// Returns the RIFF padded length for a chunk data length.
    #[must_use]
    #[inline(always)]
    pub const fn padded_len(len: usize) -> Option<usize> {
        len.checked_add(Self::pad_len(len))
    }

    /// Reads the first chunk from `bytes`.
    pub const fn chunk(bytes: &[u8]) -> Result<RiffChunk<'_>, RiffError> {
        let (chunk, _) = unwrap![ok? Self::chunk_at(bytes, 0)];
        Ok(chunk)
    }
    /// Reads the root RIFF chunk.
    ///
    /// This accepts only little-endian `RIFF` for now.
    pub const fn root(bytes: &[u8]) -> Result<RiffChunk<'_>, RiffError> {
        let chunk = unwrap![ok? Self::chunk(bytes)];
        is! { chunk.id.eq(Self::RIFX), return Err(RiffError::UnsupportedRifx) }
        is! { !chunk.id.eq(Self::RIFF), return Err(RiffError::NotRiff) }
        is! { chunk.data.len() < BinTag4::LEN, return Err(RiffError::MissingContainerType) }
        Ok(chunk)
    }
    /// Iterates over chunks in a RIFF chunk region.
    #[must_use]
    #[inline(always)]
    pub const fn chunks<'a>(bytes: &'a [u8]) -> RiffChunkIter<'a> {
        RiffChunkIter::new(bytes)
    }
    pub(super) const fn chunk_at<'a>(
        bytes: &'a [u8],
        offset: usize,
    ) -> Result<(RiffChunk<'a>, usize), RiffError> {
        let header_end = unwrap![some_ok_or? offset.checked_add(8), RiffError::Overflow];
        is! { header_end > bytes.len(), return Err(RiffError::TruncatedHeader) }
        let id =
            BinTag4::new([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
        let size = u32::from_le_bytes([
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
        ]);
        let len = cfg_select! { any(target_pointer_width = "8", target_pointer_width = "16") => {
            unwrap![ok_or? crate::cast![checked size => usize], RiffError::Overflow]
            } _ => { size as usize }
        };
        let data_end = unwrap![some_ok_or? header_end.checked_add(len), RiffError::Overflow];
        is! { data_end > bytes.len(), return Err(RiffError::TruncatedData) }
        let next = unwrap![some_ok_or? data_end.checked_add(Self::pad_len(len)),
            RiffError::Overflow];
        is! { next > bytes.len(), return Err(RiffError::TruncatedPad) }
        let data = slice![bytes, header_end, ..data_end];
        Ok((RiffChunk { id, size, data, offset }, next))
    }
}
