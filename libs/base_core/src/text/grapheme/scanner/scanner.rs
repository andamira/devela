// devela_base_core::text::grapheme::scanner::scanner
//
//!
//
// TOC
// - impl over char_utf8
// - impl over char

use crate::{
    GraphemeBoundary, GraphemeMachine, IteratorFused, PhantomData, StringNonul, StringU8,
    char_utf8, is, slice, unwrap,
};

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_ITERATOR!()]
/// Scans text and detects grapheme cluster boundaries during iteration.
///
#[doc = crate::_doc!(location_item: "text/grapheme/struct.GraphemeScanner.html")]
///
/// Can process different text representations (`&str`, `&[u8]`) while
/// tracking grapheme cluster boundaries through a `GraphemeMachine`.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GraphemeScanner<'a, C> {
    machine: &'a mut GraphemeMachine,
    remain: &'a str,
    _char: PhantomData<C>,
}
impl<'a> GraphemeScanner<'a, char_utf8> {
    /// Creates a new grapheme iterator over the remaining text.
    pub const fn new(machine: &'a mut GraphemeMachine, remain: &'a str) -> Self {
        GraphemeScanner::<char_utf8> { machine, remain, _char: PhantomData }
    }

    /// Returns the next code point and its grapheme boundary action.
    pub const fn next(&mut self) -> Option<(GraphemeBoundary, char_utf8)> {
        let (next, len) = unwrap![some? char_utf8::from_str_with_len(self.remain)];
        let action = self.machine.next_char_utf8(next);
        self.remain = slice![str self.remain, len as usize, ..];
        Some((action, next))
    }

    /// Returns the next complete grapheme cluster as a `StringU8`.
    ///
    /// Returns `None` when there are no more graphemes to process.
    /// The grapheme will be truncated if it exceeds the capacity `CAP`.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{GraphemeMachine, GraphemeScanner, StringU8, char_utf8};
    /// let input = "H€🧑‍🌾";
    /// let mut machine = GraphemeMachine::new();
    /// let mut scanner = GraphemeScanner::<char_utf8>::new(&mut machine, input);
    ///
    /// let g = scanner.next_grapheme_u8::<32>().unwrap(); assert_eq!(g.as_str(), "H");
    /// let g = scanner.next_grapheme_u8::<32>().unwrap(); assert_eq!(g.as_str(), "€");
    /// let g = scanner.next_grapheme_u8::<32>().unwrap(); assert_eq!(g.as_str(), "🧑‍🌾");
    /// assert!(scanner.next_grapheme_u8::<32>().is_none());
    /// ```
    pub const fn next_grapheme_u8<const CAP: usize>(&mut self) -> Option<StringU8<CAP>> {
        let mut g = StringU8::<CAP>::new();
        let mut buf = [0u8; 4];
        while let Some((ch, len)) = char_utf8::from_str_with_len(self.remain) {
            let boundary = self.machine.next_char_utf8(ch);
            // if split occurs return the previous grapheme:
            is![boundary.eq(GraphemeBoundary::Split) && !g.is_empty(); return Some(g)];
            // add char to current grapheme, breaking if capacity is exceeded:
            is![g.try_push_str(ch.as_str_into(&mut buf)).is_err(); break];
            self.remain = slice![str self.remain, len as usize, ..];
        }
        is![!g.is_empty(); Some(g); None]
    }

    /// Returns the next complete grapheme cluster as a `StringNonul`.
    ///
    /// Returns `None` when there are no more graphemes to process.
    /// The grapheme will be truncated if it exceeds the capacity `CAP`.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{GraphemeMachine, GraphemeScanner, char_utf8};
    /// let input = "H€🧑‍🌾";
    /// let mut machine = GraphemeMachine::new();
    /// let mut scanner = GraphemeScanner::<char_utf8>::new(&mut machine, input);
    ///
    /// let g = scanner.next_grapheme_nonul::<32>().unwrap(); assert_eq!(g.as_str(), "H");
    /// let g = scanner.next_grapheme_nonul::<32>().unwrap(); assert_eq!(g.as_str(), "€");
    /// let g = scanner.next_grapheme_nonul::<32>().unwrap(); assert_eq!(g.as_str(), "🧑‍🌾");
    /// assert!(scanner.next_grapheme_nonul::<32>().is_none());
    /// ```
    pub const fn next_grapheme_nonul<const CAP: usize>(&mut self) -> Option<StringNonul<CAP>> {
        let mut g = StringNonul::<CAP>::new();
        let mut buf = [0u8; 4];
        let mut has_content = false; // avoid costly is_empty() calls
        while let Some((ch, len)) = char_utf8::from_str_with_len(self.remain) {
            let boundary = self.machine.next_char_utf8(ch);
            // if split occurs return the previous grapheme:
            is![boundary.eq(GraphemeBoundary::Split) && has_content; return Some(g)];
            // add char to current grapheme, breaking if capacity is exceeded:
            is![g.try_push_str(ch.as_str_into(&mut buf)).is_err(); break];
            has_content = true; // we have now at least 1 code point
            self.remain = slice![str self.remain, len as usize, ..];
        }
        is![has_content; Some(g); None]
    }
}

impl<'a> Iterator for GraphemeScanner<'a, char_utf8> {
    type Item = (GraphemeBoundary, char_utf8);
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
impl<'a> IteratorFused for GraphemeScanner<'a, char_utf8> {}

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
        let (next, len) = unwrap![some? char_utf8::from_str_with_len(self.remain)];
        let action = self.machine.next_char_utf8(next);
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
