// devela/src/data/codec/pack/wrap/riff/chunk.rs
//
//! Defines [`RiffChunk`], [`RiffChunkIter`].
//

use crate::{BinTag4, Riff, RiffError, is, slice};

#[doc = crate::_tags!(data codec)]
/// A borrowed RIFF chunk.
#[doc = crate::_doc_meta!{
    location("data/codec/pack", struct RiffChunk),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RiffChunk = 20|160; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RiffChunk = 32|256; niche Option),
}]
/// This is a view into an existing byte slice. The chunk owns no data and does
/// not interpret its payload unless asked for container-specific information
/// such as its RIFF form type or LIST type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RiffChunk<'a> {
    pub(super) id: BinTag4,
    pub(super) size: u32,
    pub(super) data: &'a [u8],
    pub(super) offset: usize,
}

#[rustfmt::skip]
impl<'a> RiffChunk<'a> {
    /// Returns the chunk identifier.
    pub const fn id(self) -> BinTag4 { self.id }

    #[must_use]
    /// Returns the declared chunk data size.
    pub const fn size(self) -> u32 { self.size }
    #[must_use]
    /// Returns the chunk data length.
    pub const fn len(self) -> usize { self.data.len() }
    #[must_use]
    /// Returns whether the chunk data is empty.
    pub const fn is_empty(self) -> bool { self.data.is_empty() }

    #[must_use]
    /// Returns the chunk data.
    pub const fn data(self) -> &'a [u8] { self.data }
    #[must_use]
    /// Returns the chunk offset relative to the parsed byte region.
    pub const fn offset(self) -> usize { self.offset }

    #[must_use]
    /// Returns whether this chunk is `RIFF`.
    pub const fn is_riff(self) -> bool { self.id.eq_bytes(*b"RIFF") }
    #[must_use]
    /// Returns whether this chunk is `RIFX`.
    pub const fn is_rifx(self) -> bool { self.id.eq_bytes(*b"RIFX") }
    #[must_use]
    /// Returns whether this chunk is `LIST`.
    pub const fn is_list(self) -> bool { self.id.eq_bytes(*b"LIST") }

    #[must_use]
    /// Returns whether this chunk can contain subchunks.
    pub const fn is_container(self) -> bool {
        self.is_riff() || self.is_rifx() || self.is_list()
    }
    #[must_use]
    /// Returns the RIFF form type or LIST type.
    pub const fn container_type(self) -> Option<BinTag4> {
        is! { !self.is_container() || self.data.len() < 4, return None }
        Some(BinTag4::new([self.data[0], self.data[1], self.data[2], self.data[3]]))
    }

    #[must_use]
    /// Returns the subchunk byte region of a `RIFF`, `RIFX`, or `LIST` chunk.
    pub const fn subchunk_data(self) -> Option<&'a [u8]> {
        is! { !self.is_container() || self.data.len() < 4, return None }
        Some(self.data.split_at(4).1)
    }
    /// Iterates over this chunk's subchunks.
    pub const fn subchunks(self) -> Result<RiffChunkIter<'a>, RiffError> {
        let Some(data) = self.subchunk_data() else { return Err(RiffError::MissingContainerType); };
        Ok(Riff::chunks(data))
    }
}

#[must_use]
#[doc = crate::_tags!(data codec iterator)]
/// An iterator over borrowed RIFF chunks.
#[doc = crate::_doc_meta!{
    location("data/codec/pack", struct RiffChunkIter),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RiffChunkIter = 12|96; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RiffChunkIter = 24|192; niche Option),
}]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RiffChunkIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}
#[rustfmt::skip]
impl<'a> RiffChunkIter<'a> {
    pub(crate) const fn _new(bytes: &'a [u8], offset: usize) -> Self { Self { bytes, offset } }

    /// Returns a new iterator over the chunks in `bytes`.
    pub const fn new(bytes: &'a [u8]) -> Self { Self::_new(bytes, 0) }

    #[must_use]
    /// Returns the remaining byte region.
    pub const fn remaining(self) -> &'a [u8] { slice![self.bytes, self.offset, ..] }
    #[must_use]
    /// Returns the current byte offset.
    pub const fn offset(self) -> usize { self.offset }
    #[must_use]
    /// Returns whether no bytes remain.
    pub const fn is_empty(self) -> bool { self.offset >= self.bytes.len() }

    #[must_use]
    /// Returns the next RIFF chunk.
    ///
    /// Returns `None` once there are no bytes left.
    ///
    /// On error, the iterator is exhausted.
    pub const fn next_chunk(&mut self) -> Option<Result<RiffChunk<'a>, RiffError>> {
        is![self.offset >= self.bytes.len(), return None];
        match Riff::chunk_at(self.bytes, self.offset) {
            Ok((chunk, next)) => { self.offset = next; Some(Ok(chunk)) }
            Err(err) => { self.offset = self.bytes.len(); Some(Err(err)) }
        }
    }
    /// Counts remaining valid chunks, consuming the iterator.
    ///
    /// # Errors
    /// Returns the first parsing error encountered.
    pub const fn count_chunks(mut self) -> Result<usize, RiffError> {
        let mut count = 0;
        while let Some(result) = self.next_chunk() {
            match result { Ok(_) => count += 1, Err(err) => return Err(err) }
        }
        Ok(count)
    }
}
#[rustfmt::skip]
impl<'a> Iterator for RiffChunkIter<'a> {
    type Item = Result<RiffChunk<'a>, RiffError>;
    fn next(&mut self) -> Option<Self::Item> { self.next_chunk() }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.bytes.len().saturating_sub(self.offset);
        // Every valid chunk needs at least 8 bytes of header.
        // If trailing invalid bytes remain, `next_chunk` can still yield one error.
        let max = if remaining == 0 { 0 } else { remaining / 8 + 1 };
        (0, Some(max))
    }
}
impl<'a> crate::IteratorFused for RiffChunkIter<'a> {}
