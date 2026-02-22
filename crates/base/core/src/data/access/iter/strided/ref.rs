// devela_base_core::data::access::iter::strided::ref
//
//! Defines [`StridedIter`].
//
// TOC
// - (struct StridedIterCore)
// - struct StridedIter
// - struct StridedIterMut

use super::StridedIterCore;
use crate::{
    IteratorLending, IteratorLendingDoubleEnded, IteratorLendingExactSize, IteratorLendingPeek,
    IteratorLendingPeekDoubleEnded, NonZeroUsize, unwrap,
};

#[doc = crate::_tags!(iterator)]
/// Iterates over a slice using an affine index progression.
#[doc = crate::_doc_location!("data/access/iter")]
///
/// This is the immutable counterpart of [`StridedIterMut`].
///
/// Successive elements are accessed according to:
///
/// `index_k = front + k * stride`
///
/// for increasing `k`, until the inclusive bound `back` is reached.
///
/// The iterator supports forward and backward traversal
/// via [`DoubleEndedIterator`].
///
/// This type is dimension-agnostic. It can be used to:
/// - Traverse rows, columns, or diagonals of a 2D layout.
/// - Project channels from interleaved buffers (e.g. RGBRGB…).
/// - Downsample a sequence by stepping every `stride` elements.
/// - Traverse a collapsed axis of an n-dimensional layout.
///
/// # Invariants
/// - `front <= back`, or the iterator is empty.
/// - All generated indices must lie within `storage`.
///
/// Violating these conditions will cause a panic when advanced.
/// No unsafe code is used.
pub struct StridedIter<'a, T> {
    storage: &'a [T],
    core: StridedIterCore,
}
impl<'a, T> StridedIter<'a, T> {
    /* contructors */

    /// Creates a strided iterator
    /// from a starting index, a number of elements, and a stride.
    ///
    /// Generates indices:
    /// `start + k * stride` for `k` in `0..remaining`.
    ///
    /// # Panics
    /// Panics if `stride == 0`.
    /// May panic when advanced if the generated indices are out of bounds.
    pub const fn from_count(
        storage: &'a [T],
        start: usize,
        remaining: usize,
        stride: usize,
    ) -> Self {
        unwrap![some_map_into_expect NonZeroUsize::new(stride),
            |v| Self::from_count_nz(storage, start, remaining, v), "stride must be > 0"]
    }
    /// Like [`from_count`][Self::from_count] but takes a non-zero stride.
    pub const fn from_count_nz(
        storage: &'a [T],
        start: usize,
        remaining: usize,
        stride: NonZeroUsize,
    ) -> Self {
        Self {
            storage,
            core: StridedIterCore::from_count(start, remaining, stride),
        }
    }

    /// Creates a strided iterator
    /// from inclusive front and back limits and a stride.
    ///
    /// Iteration proceeds from `front` toward `back` (inclusive limit)
    /// in steps of `stride`. The iterator is empty if `front > back`.
    ///
    /// # Panics
    /// Panics if `stride == 0`.
    /// May panic when advanced if the generated indices are out of bounds.
    pub const fn from_bounds(storage: &'a [T], front: usize, back: usize, stride: usize) -> Self {
        unwrap![some_map_into_expect NonZeroUsize::new(stride),
            |v| Self::from_bounds_nz(storage, front, back, v), "stride must be > 0"]
    }
    /// Like [`from_bounds`][Self::from_bounds] but takes a non-zero `stride`.
    pub const fn from_bounds_nz(
        storage: &'a [T],
        front: usize,
        back: usize,
        stride: NonZeroUsize,
    ) -> Self {
        Self {
            storage,
            core: StridedIterCore::from_bounds(front, back, stride),
        }
    }

    /* queries */

    /// Returns the number of elements remaining in the iterator.
    pub const fn len(&self) -> usize {
        self.core.len()
    }

    /* state */

    /// Advances the iterator and returns a shared reference to the next value.
    pub const fn next(&mut self) -> Option<&T> {
        Some(&self.storage[unwrap!(some? self.core.next_front_index())])
    }
    /// Returns a shared reference to the next value, without updating the iterator.
    pub const fn peek(&self) -> Option<&T> {
        Some(&self.storage[unwrap!(some? self.core.peek_next_front_index())])
    }
    /// Advances the iterator and returns a shared reference to the next value from the back.
    pub const fn next_back(&mut self) -> Option<&T> {
        Some(&self.storage[unwrap!(some? self.core.next_back_index())])
    }
    /// Returns a shared reference to the next value from the back, without updating the iterator.
    pub const fn peek_back(&self) -> Option<&T> {
        Some(&self.storage[unwrap!(some? self.core.peek_next_back_index())])
    }
}
impl<'a, T> Iterator for StridedIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(&self.storage[self.core.next_front_index()?])
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a, T> DoubleEndedIterator for StridedIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(&self.storage[self.core.next_back_index()?])
    }
}
impl<T> ExactSizeIterator for StridedIter<'_, T> {}
impl<'a, T> IteratorLending for StridedIter<'a, T> {
    type Item<'b>
        = &'b T
    where
        Self: 'b;
    fn next<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        self.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a, T> IteratorLendingExactSize for StridedIter<'a, T> {}
impl<'a, T> IteratorLendingPeek for StridedIter<'a, T> {
    fn peek<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        Self::peek(self)
    }
}
impl<'a, T> IteratorLendingDoubleEnded for StridedIter<'a, T> {
    fn next_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        Some(&self.storage[self.core.next_back_index()?])
    }
}
impl<'a, T> IteratorLendingPeekDoubleEnded for StridedIter<'a, T> {
    fn peek_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        Self::peek_back(self)
    }
}
