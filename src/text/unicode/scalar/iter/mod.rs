// devela/src/text/unicode/scalar/iter/mod.rs
//
//! Defines the [`CharIter`] iterator.
//

use crate::{IteratorFused, PhantomData, slice};

mod bytes; // methods over &[u8]
mod str; // methods over &str

mod layout; // common methods related to text-layout

#[doc = crate::_tags!(text iterator)]
/// An iterator over Unicode scalars.
#[doc = crate::_doc_meta!{location("text/unicode/scalar")}]
///
/// Implements `Iterator<Item = char>` by default, but provides specialized methods
/// for other scalar types.
///
/// For grapheme-level iteration see [`GraphemeIter`].
///
/// [`GraphemeIter`]: crate::GraphemeIter
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharIter<'a, Source> {
    bytes: &'a [u8],
    pos: usize,
    _source: PhantomData<Source>,
}
impl<'a, Source> CharIter<'a, Source> {
    pub(crate) const fn _new(bytes: &'a [u8], pos: usize) -> Self {
        Self { bytes, pos, _source: PhantomData }
    }
    /// Returns the current byte position.
    #[must_use]
    pub const fn byte_pos(&self) -> usize {
        self.pos
    }
    /// Returns the remaining bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &'a [u8] {
        slice![self.bytes, self.pos, ..]
    }
    /// Returns the number of remaining bytes.
    #[must_use]
    pub const fn remaining_bytes(&self) -> usize {
        self.bytes.len() - self.pos
    }
    /// Returns `true` if there are no more bytes to decode.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.pos >= self.bytes.len()
    }
}

impl<'a> Iterator for CharIter<'a, &'a str> {
    type Item = char;

    #[inline(always)]
    fn next(&mut self) -> Option<char> {
        self.next_char()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.bytes.len() - self.pos;
        (remaining.div_ceil(4), Some(remaining))
    }
    #[inline(always)]
    fn count(self) -> usize {
        self.count()
    }
}
impl<'a> IteratorFused for CharIter<'a, &'a str> {}

impl<'a> Iterator for CharIter<'a, &'a [u8]> {
    type Item = char;

    #[inline(always)]
    fn next(&mut self) -> Option<char> {
        self.next_char()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.bytes.len() - self.pos;
        (remaining.div_ceil(4), Some(remaining))
    }
    #[inline(always)]
    fn count(self) -> usize {
        self.count()
    }
}
impl<'a> IteratorFused for CharIter<'a, &'a [u8]> {}
