// devela/src/text/unicode/grapheme/scanner/iter.rs
//
//! Defines [`GraphemeIter`], an owned grapheme boundary iterator.
//
// TOC
// - struct GraphemeIter
// - impl over charu
// - impl over char

use crate::{
    GraphemeBoundary, GraphemeMachine, GraphemeNonul, GraphemeU8, IteratorFused, PhantomData,
    StringNonul, StringU8, charu, is, slice, unwrap,
};

#[doc = crate::_tags!(text iterator)]
/// Owns grapheme segmentation state while scanning a string slice.
#[doc = crate::_doc_meta!{
    location("text/unicode/grapheme"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: GraphemeIter<'_, char> = 12|96; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: GraphemeIter<'_, char> = 24|192; niche Option),
}]
///
/// This is the ergonomic single-buffer counterpart to [`GraphemeScanner`][crate::GraphemeScanner].
/// Use `GraphemeScanner` when a caller-provided [`GraphemeMachine`] must
/// continue across multiple input buffers.
///
/// For scalar-level iteration see [`CharIter`][crate::CharIter].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphemeIter<'a, C = char> {
    machine: GraphemeMachine,
    remain: &'a str,
    _char: PhantomData<C>,
}

impl<'a> GraphemeIter<'a, charu> {
    /// Creates a new grapheme iterator over `string`.
    #[must_use]
    pub const fn new_charu(string: &'a str) -> Self {
        Self {
            machine: GraphemeMachine::new(),
            remain: string,
            _char: PhantomData,
        }
    }
    /// Returns the next code point and its grapheme boundary action.
    pub const fn next(&mut self) -> Option<(GraphemeBoundary, charu)> {
        let (next, len) = unwrap![some? charu::from_str_with_len(self.remain)];
        let action = self.machine.next_charu(next);
        self.remain = slice![str self.remain, len as usize, ..];
        Some((action, next))
    }
    /// Returns the remaining unscanned string slice.
    #[must_use]
    pub const fn remaining(&self) -> &'a str {
        self.remain
    }
    /// Returns the current segmentation machine state.
    #[must_use]
    pub const fn machine(&self) -> GraphemeMachine {
        self.machine
    }
    /// Returns the next complete grapheme cluster as a `GraphemeU8`.
    ///
    /// Returns `None` when there are no more graphemes to process.
    /// The grapheme will be truncated if it exceeds the capacity `CAP`.
    ///
    /// # Panics
    /// Panics if `CAP > 255`.
    pub const fn next_grapheme_u8<const CAP: usize>(&mut self) -> Option<GraphemeU8<CAP>> {
        let mut g = StringU8::<CAP>::new();
        let mut buf = [0u8; 4];
        while let Some((ch, len)) = charu::from_str_with_len(self.remain) {
            let boundary = self.machine.next_charu(ch);
            if boundary.eq(GraphemeBoundary::Split) && !g.is_empty() {
                return Some(GraphemeU8(g));
            }
            is![g.try_push_str(ch.as_str_into(&mut buf)).is_err(), break];
            self.remain = slice![str self.remain, len as usize, ..];
        }
        is![!g.is_empty(), Some(GraphemeU8(g)), None]
    }

    /// Returns the next complete grapheme cluster as a `GraphemeNonul`.
    ///
    /// Returns `None` when there are no more graphemes to process.
    /// The grapheme will be truncated if it exceeds the capacity `CAP`.
    pub const fn next_grapheme_nonul<const CAP: usize>(&mut self) -> Option<GraphemeNonul<CAP>> {
        let mut g = StringNonul::<CAP>::new();
        let mut buf = [0u8; 4];
        let mut has_content = false;
        while let Some((ch, len)) = charu::from_str_with_len(self.remain) {
            let boundary = self.machine.next_charu(ch);
            if boundary.eq(GraphemeBoundary::Split) && !g.is_empty() {
                return Some(GraphemeNonul(g));
            }
            is![g.try_push_str(ch.as_str_into(&mut buf)).is_err(), break];
            has_content = true;
            self.remain = slice![str self.remain, len as usize, ..];
        }
        is![has_content, Some(GraphemeNonul(g)), None]
    }
}
impl<'a> Iterator for GraphemeIter<'a, charu> {
    type Item = (GraphemeBoundary, charu);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
impl<'a> IteratorFused for GraphemeIter<'a, charu> {}

/* char */

impl<'a> GraphemeIter<'a, char> {
    /// Creates a new grapheme iterator over `string`.
    #[must_use]
    pub const fn new(string: &'a str) -> Self {
        Self {
            machine: GraphemeMachine::new(),
            remain: string,
            _char: PhantomData,
        }
    }
    /// Returns the next code point and its grapheme boundary action.
    pub const fn next(&mut self) -> Option<(GraphemeBoundary, char)> {
        let (next, len) = unwrap![some? charu::from_str_with_len(self.remain)];
        let action = self.machine.next_charu(next);
        self.remain = slice![str self.remain, len as usize, ..];
        Some((action, next.to_char()))
    }
    /// Returns the remaining unscanned string slice.
    #[must_use]
    pub const fn remaining(&self) -> &'a str {
        self.remain
    }
    /// Returns the current segmentation machine state.
    #[must_use]
    pub const fn machine(&self) -> GraphemeMachine {
        self.machine
    }
}

impl<'a> Iterator for GraphemeIter<'a, char> {
    type Item = (GraphemeBoundary, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
impl<'a> IteratorFused for GraphemeIter<'a, char> {}
