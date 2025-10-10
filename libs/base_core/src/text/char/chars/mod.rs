// devela::text::char::chars
//
//! Defines the [`CharIter`] iterator.
//

use crate::{IteratorFused, PhantomData};

mod str; // methods over &str
mod bytes; // methods over &[u8]

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_ITERATOR!()]
/// An iterator over Unicode scalars.
#[doc = crate::_doc!(location_item: "text/char/struct.CharIter.html")]
///
/// Implements `Iterator<Item = char>` by default, but provides specialized methods
/// for other scalar types.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharIter<'a, Source> {
    bytes: &'a [u8],
    pos: usize,
    _source: PhantomData<Source>,
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
