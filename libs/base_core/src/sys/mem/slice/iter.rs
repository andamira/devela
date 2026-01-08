// devela_base_core::sys::mem::slice::iter
//
//! Defines [`SliceIter`], [`SliceIterMut`].
//

use crate::{
    IteratorLending, IteratorLendingDoubleEnded, IteratorLendingExactSize, IteratorLendingPeek,
};

#[doc = crate::_tags!(iterator lifetime)]
/// A lending iterator over a shared slice.
#[doc = crate::_doc_location!("sys/mem")]
///
/// Yields elements by reference in index order. Produces `&T` without copying.
#[derive(Debug)]
pub struct SliceIter<'s, T> {
    slice: &'s [T],
    front: usize,
    back: usize,
}
impl<'s, T> SliceIter<'s, T> {
    /// Creates an iterator over the given shared slice starting at index `0`.
    pub fn new(slice: &'s [T]) -> Self {
        let len = slice.len();
        Self { slice, front: 0, back: len }
    }
}
impl<'s, T> IteratorLending for SliceIter<'s, T> {
    type Item<'a>
        = &'a T
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.back {
            None
        } else {
            let i = self.front;
            self.front += 1;
            Some(&self.slice[i])
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back - self.front;
        (len, Some(len))
    }
}
impl<'s, T> IteratorLendingExactSize for SliceIter<'s, T> {}
impl<'s, T> IteratorLendingDoubleEnded for SliceIter<'s, T> {
    fn next_back<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.back {
            None
        } else {
            self.back -= 1;
            Some(&self.slice[self.back])
        }
    }
}
impl<'s, T> IteratorLendingPeek for SliceIter<'s, T> {
    fn peek<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.slice.len() {
            None
        } else {
            Some(&self.slice[self.front])
        }
    }
}

//

#[doc = crate::_tags!(iterator lifetime)]
/// A lending iterator over an exclusive slice.
#[doc = crate::_doc_location!("sys/mem")]
///
/// Yields `&mut T` references in index order.
/// The returned references may outlive the iterator borrow taken by `next`
/// and may coexist as long as the underlying slice is not accessed through other means.
///
/// # Example
/// ```
/// # use devela_base_core::{IteratorLending, SliceIterMut};
/// let mut array = [0, 1, 2, 3];
/// let mut it = SliceIterMut::new(&mut array);
/// while let Some(x) = it.next() {
///     *x += 1;
/// }
/// assert_eq![array.as_slice(), &[1, 2, 3, 4]];
/// ```
#[derive(Debug)]
pub struct SliceIterMut<'s, T> {
    slice: &'s mut [T],
    front: usize,
    back: usize,
}
impl<'s, T> SliceIterMut<'s, T> {
    /// Creates an iterator over the given exclusive slice starting at index `0`.
    pub fn new(slice: &'s mut [T]) -> Self {
        let len = slice.len();
        Self { slice, front: 0, back: len }
    }
}
impl<'s, T> IteratorLending for SliceIterMut<'s, T> {
    type Item<'a>
        = &'a mut T
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.back {
            None
        } else {
            let i = self.front;
            self.front += 1;
            Some(&mut self.slice[i])
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back - self.front;
        (len, Some(len))
    }
}
impl<'s, T> IteratorLendingExactSize for SliceIterMut<'s, T> {}
impl<'s, T> IteratorLendingDoubleEnded for SliceIterMut<'s, T> {
    fn next_back<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.front >= self.back {
            None
        } else {
            self.back -= 1;
            Some(&mut self.slice[self.back])
        }
    }
}
