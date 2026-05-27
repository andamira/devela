// devela::data::codec::pack::wrap::riff::namespace
//
//! Defines [`Riff`].
//

use crate::{BinTag4, RiffChunk, RiffChunkIter, RiffError, is, slice, unwrap, write_at};

#[doc = crate::_tags!(data codec)]
/// RIFF tagged binary container operations.
#[doc = crate::_doc_meta!{location("data/codec/pack")}]
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
    /// RIFF chunk header length.
    pub const CHUNK_HEADER_LEN: usize = 8;
    /// RIFF form/list type length.
    pub const FORM_TYPE_LEN: usize = 4;
    /// RIFF root header length including the form type.
    pub const ROOT_HEADER_LEN: usize = Self::CHUNK_HEADER_LEN + Self::FORM_TYPE_LEN;

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
    /// Iterates over chunks in a RIFF chunk region.
    #[inline(always)]
    pub const fn chunks<'a>(bytes: &'a [u8]) -> RiffChunkIter<'a> {
        RiffChunkIter::new(bytes)
    }
    /// Returns the full byte length of a chunk, including header and padding.
    #[must_use]
    pub const fn chunk_len(data_len: usize) -> Option<usize> {
        let len = unwrap![some? Self::CHUNK_HEADER_LEN.checked_add(data_len)];
        len.checked_add(Self::pad_len(data_len))
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

    /// Returns the full byte length of a RIFF form from its subchunk byte length.
    ///
    /// This includes:
    /// - 8-byte `RIFF` chunk header
    /// - 4-byte form type
    /// - all subchunks
    #[must_use]
    pub const fn form_len(subchunks_len: usize) -> Option<usize> {
        Self::ROOT_HEADER_LEN.checked_add(subchunks_len)
    }
    /// Returns whether a RIFF chunk size field can represent `data_len`.
    #[must_use]
    pub const fn fits_u32(data_len: usize) -> bool {
        data_len <= u32::MAX as usize
    }

    /// Writes a RIFF chunk header at the start of `dst`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// Returns [`RiffError::NotEnoughSpace`] if `dst` is shorter than 8 bytes.
    /// Returns [`RiffError::Overflow`] if `data_len` does not fit in a RIFF
    /// 32-bit chunk size field.
    pub const fn write_chunk_header(
        dst: &mut [u8],
        id: BinTag4,
        data_len: usize,
    ) -> Result<usize, RiffError> {
        is! { dst.len() < Self::CHUNK_HEADER_LEN, return Err(RiffError::NotEnoughSpace) }
        is! { !Self::fits_u32(data_len), return Err(RiffError::Overflow) }
        let size = (data_len as u32).to_le_bytes();
        let id = id.bytes();
        write_at![dst, 0, id[0], id[1], id[2], id[3], size[0], size[1], size[2], size[3]];
        Ok(Self::CHUNK_HEADER_LEN)
    }

    /// Writes a RIFF root header for a form such as `WAVE`.
    ///
    /// `subchunks_len` is the total encoded length of the form's subchunks.
    /// The RIFF size field written is `4 + subchunks_len`, because the form
    /// type is part of the RIFF chunk payload.
    ///
    /// Returns the number of bytes written.
    pub const fn write_form_header(
        dst: &mut [u8],
        form: BinTag4,
        subchunks_len: usize,
    ) -> Result<usize, RiffError> {
        let Some(riff_data_len) = Self::FORM_TYPE_LEN.checked_add(subchunks_len) else {
            return Err(RiffError::Overflow);
        };
        is! { dst.len() < Self::ROOT_HEADER_LEN, return Err(RiffError::NotEnoughSpace) }
        let mut offset = unwrap![ok? Self::write_chunk_header(dst, Self::RIFF, riff_data_len)];
        let form = form.bytes();
        write_at![dst, +=offset, form[0], form[1], form[2], form[3]];
        Ok(offset)
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
/// Common RIFF tags.
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
    /// Common WAVE fact chunk.
    pub const FACT: BinTag4 = BinTag4::new(*b"fact");
    /// Common binary data chunk.
    pub const DATA: BinTag4 = BinTag4::new(*b"data");
    /// Common filler chunk.
    pub const JUNK: BinTag4 = BinTag4::new(*b"JUNK");
    /// Common associated-info list type.
    pub const INFO: BinTag4 = BinTag4::new(*b"INFO");
}
