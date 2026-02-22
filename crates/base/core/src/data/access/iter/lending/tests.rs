// devela_base_core::data::access::iter::lending::tests

use super::*;

struct SliceLend<'s, T> {
    s: &'s [T],
    front: usize,
    back: usize,
}

impl<'s, T> SliceLend<'s, T> {
    fn new(s: &'s [T]) -> Self {
        Self { s, front: 0, back: s.len() }
    }
    fn rem(&self) -> usize {
        self.back.saturating_sub(self.front)
    }
}

impl<'s, T> IteratorLending for SliceLend<'s, T> {
    type Item<'a>
        = &'a T
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.back {
            return None;
        }
        let i = self.front;
        self.front += 1;
        Some(&self.s[i])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.rem();
        (n, Some(n))
    }
}

impl<'s, T> IteratorLendingDoubleEnded for SliceLend<'s, T> {
    fn next_back<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(&self.s[self.back])
    }
}

impl<'s, T> IteratorLendingExactSize for SliceLend<'s, T> {}

impl<'s, T> IteratorLendingPeek for SliceLend<'s, T> {
    fn peek<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.back { None } else { Some(&self.s[self.front]) }
    }
}

impl<'s, T> IteratorLendingPeekDoubleEnded for SliceLend<'s, T> {
    fn peek_back<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.back { None } else { Some(&self.s[self.back - 1]) }
    }
}

/* core */

#[test]
fn next_and_count() {
    let xs = [10, 20, 30];
    let mut it = SliceLend::new(&xs);
    assert_eq!(it.next(), Some(&10));
    assert_eq!(it.count(), 2);
    assert_eq!(it.next(), None);
}

#[test]
fn advance_by() {
    let xs = [1, 2, 3];
    let mut it = SliceLend::new(&xs);
    assert_eq!(it.advance_by(2), Ok(()));
    assert_eq!(it.next(), Some(&3));

    let mut it = SliceLend::new(&xs);
    assert_eq!(it.advance_by(5), Err(2)); // 5-3 = 2 remaining not skipped
}

#[test]
fn nth() {
    let xs = [5, 6, 7, 8];
    let mut it = SliceLend::new(&xs);
    assert_eq!(it.nth(2), Some(&7));
    assert_eq!(it.next(), Some(&8));
    assert_eq!(it.next(), None);
}

/* double ended */

#[test]
fn next_back_and_mix() {
    let xs = [1, 2, 3, 4];
    let mut it = SliceLend::new(&xs);
    assert_eq!(it.next_back(), Some(&4));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next_back(), Some(&3));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next_back(), None);
}

#[test]
fn nth_back() {
    let xs = [10, 11, 12, 13];
    let mut it = SliceLend::new(&xs);
    assert_eq!(it.nth_back(1), Some(&12));
    assert_eq!(it.next_back(), Some(&11));
}

/* exact size */

#[test]
fn len_tracks_remaining() {
    let xs = [1, 2, 3];
    let mut it = SliceLend::new(&xs);
    assert_eq!(it.len(), 3);
    it.next();
    assert_eq!(it.len(), 2);
    it.next_back();
    assert_eq!(it.len(), 1);
    it.next();
    assert!(it.is_empty());
}

/* peek */

#[test]
fn peek_does_not_advance() {
    let xs = [9, 8, 7];
    let mut it = SliceLend::new(&xs);
    assert_eq!(it.peek(), Some(&9));
    assert_eq!(it.peek(), Some(&9));
    assert_eq!(it.next(), Some(&9));
}

// FIXME
// #[test]
// fn next_if_predicate() {
//     let xs = [1, 2, 3];
//     let mut it = SliceLend::new(&xs);
//     assert_eq!(it.next_if(|x| **x == 2), None);
//     assert_eq!(it.next(), Some(&1));
//     assert_eq!(it.next_if(|x| **x == 2), Some(&2));
//     assert_eq!(it.next(), Some(&3));
// }

/* peek back */

#[test]
fn peek_back_does_not_advance() {
    let xs = [1, 2, 3];
    let mut it = SliceLend::new(&xs);
    assert_eq!(it.peek_back(), Some(&3));
    assert_eq!(it.peek_back(), Some(&3));
    assert_eq!(it.next_back(), Some(&3));
}

// FIXME
// #[test]
// fn next_back_if_eq() {
//     let xs = [1, 2, 3];
//     let mut it = SliceLend::new(&xs);
//     assert_eq!(it.next_back_if_eq(&2), None);
//     // assert_eq!(it.next_back(), Some(&3));
//     // assert_eq!(it.next_back_if_eq(&2), Some(&2));
//     // assert_eq!(it.next_back(), Some(&1));
// }
