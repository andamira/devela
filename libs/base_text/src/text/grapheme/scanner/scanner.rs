// devela_base_text::text::grapheme::scanner::scanner
//
//!
//
// TOC
// - impl over charu
// - impl over char

use crate::{
    GraphemeBoundary, GraphemeMachine, GraphemeNonul, GraphemeU8, IteratorFused, PhantomData,
    StringNonul, StringU8, charu, is, slice, unwrap,
};

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_ITERATOR!()]
/// Scans text and detects grapheme cluster boundaries during iteration.
///
#[doc = crate::_doc!(location: "text/grapheme")]
///
/// Can process different text representations (`&str`, `&[u8]`) while
/// tracking grapheme cluster boundaries through a `GraphemeMachine`.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GraphemeScanner<'a, C> {
    machine: &'a mut GraphemeMachine,
    remain: &'a str,
    _char: PhantomData<C>,
}
impl<'a> GraphemeScanner<'a, charu> {
    /// Creates a new grapheme iterator over the remaining text.
    pub const fn new(machine: &'a mut GraphemeMachine, remain: &'a str) -> Self {
        GraphemeScanner::<charu> { machine, remain, _char: PhantomData }
    }

    /// Returns the next code point and its grapheme boundary action.
    pub const fn next(&mut self) -> Option<(GraphemeBoundary, charu)> {
        let (next, len) = unwrap![some? charu::from_str_with_len(self.remain)];
        let action = self.machine.next_charu(next);
        self.remain = slice![str self.remain, len as usize, ..];
        Some((action, next))
    }

    /// Returns the next complete grapheme cluster as a `GraphemeU8`.
    ///
    /// Returns `None` when there are no more graphemes to process.
    /// The grapheme will be truncated if it exceeds the capacity `CAP`.
    ///
    /// # Panics
    /// Panics if `CAP > 255.
    ///
    /// # Example
    /// ```
    /// # use devela_base_text::{GraphemeMachine, GraphemeScanner, GraphemeU8, charu};
    /// let input = "H€🧑‍🌾";
    /// let mut machine = GraphemeMachine::new();
    /// let mut scanner = GraphemeScanner::<charu>::new(&mut machine, input);
    ///
    /// let g = scanner.next_grapheme_u8::<32>().unwrap(); assert_eq!(g.as_str(), "H");
    /// let g = scanner.next_grapheme_u8::<32>().unwrap(); assert_eq!(g.as_str(), "€");
    /// let g = scanner.next_grapheme_u8::<32>().unwrap(); assert_eq!(g.as_str(), "🧑‍🌾");
    /// assert!(scanner.next_grapheme_u8::<32>().is_none());
    /// ```
    pub const fn next_grapheme_u8<const CAP: usize>(&mut self) -> Option<GraphemeU8<CAP>> {
        let mut g = StringU8::<CAP>::new();
        let mut buf = [0u8; 4];
        while let Some((ch, len)) = charu::from_str_with_len(self.remain) {
            let boundary = self.machine.next_charu(ch);
            if boundary.eq(GraphemeBoundary::Split) && !g.is_empty() {
                return Some(GraphemeU8(g)); // if split occurs return the previous grapheme
            }
            // add char to current grapheme, breaking if capacity is exceeded
            is![g.try_push_str(ch.as_str_into(&mut buf)).is_err(); break];
            self.remain = slice![str self.remain, len as usize, ..];
        }
        is![!g.is_empty(); Some(GraphemeU8(g)); None]
    }
    // TODO make another version exact non-truncating.

    /// Returns the next complete grapheme cluster as a `GraphemeNonul`.
    ///
    /// Returns `None` when there are no more graphemes to process.
    /// The grapheme will be truncated if it exceeds the capacity `CAP`.
    ///
    /// # Example
    /// ```
    /// # use devela_base_text::{GraphemeMachine, GraphemeScanner, charu};
    /// let input = "H€🧑‍🌾";
    /// let mut machine = GraphemeMachine::new();
    /// let mut scanner = GraphemeScanner::<charu>::new(&mut machine, input);
    ///
    /// let g = scanner.next_grapheme_nonul::<32>().unwrap(); assert_eq!(g.as_str(), "H");
    /// let g = scanner.next_grapheme_nonul::<32>().unwrap(); assert_eq!(g.as_str(), "€");
    /// let g = scanner.next_grapheme_nonul::<32>().unwrap(); assert_eq!(g.as_str(), "🧑‍🌾");
    /// assert!(scanner.next_grapheme_nonul::<32>().is_none());
    /// ```
    pub const fn next_grapheme_nonul<const CAP: usize>(&mut self) -> Option<GraphemeNonul<CAP>> {
        let mut g = StringNonul::<CAP>::new();
        let mut buf = [0u8; 4];
        let mut has_content = false; // to avoid costly is_empty() calls
        while let Some((ch, len)) = charu::from_str_with_len(self.remain) {
            let boundary = self.machine.next_charu(ch);
            // if split occurs return the previous grapheme:
            if boundary.eq(GraphemeBoundary::Split) && !g.is_empty() {
                return Some(GraphemeNonul(g)); // if split occurs return the previous grapheme
            }
            // add char to current grapheme, breaking if capacity is exceeded
            is![g.try_push_str(ch.as_str_into(&mut buf)).is_err(); break];
            has_content = true; // we have now at least 1 code point
            self.remain = slice![str self.remain, len as usize, ..];
        }
        is![has_content; Some(GraphemeNonul(g)); None]
    }
}

impl<'a> Iterator for GraphemeScanner<'a, charu> {
    type Item = (GraphemeBoundary, charu);
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
impl<'a> IteratorFused for GraphemeScanner<'a, charu> {}

impl<'a> GraphemeScanner<'a, char> {
    /// Creates a new grapheme iterator over the remaining text.
    pub const fn new(machine: &'a mut GraphemeMachine, remain: &'a str) -> Self {
        GraphemeScanner::<char> { machine, remain, _char: PhantomData }
    }

    /// Returns the next code point and its grapheme boundary action.
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to avoid duplicated validation.
    pub const fn next(&mut self) -> Option<(GraphemeBoundary, char)> {
        let (next, len) = unwrap![some? charu::from_str_with_len(self.remain)];
        let action = self.machine.next_charu(next);
        self.remain = slice![str self.remain, len as usize, ..];
        Some((action, next.to_char()))
    }
}
impl<'a> Iterator for GraphemeScanner<'a, char> {
    type Item = (GraphemeBoundary, char);
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
impl<'a> IteratorFused for GraphemeScanner<'a, char> {}
