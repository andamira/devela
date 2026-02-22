// devela_base_core::data::access::iter::strided::mut
//
//! Defines [`StridedIter`].
//

use super::StridedIterCore;
use crate::{
    IteratorLending, IteratorLendingDoubleEnded, IteratorLendingExactSize, IteratorLendingPeek,
    IteratorLendingPeekDoubleEnded, NonZeroUsize, unwrap,
};

#[doc = crate::_tags!(iterator)]
/// Iterates mutably over a slice using an affine index progression.
#[doc = crate::_doc_location!("data/access/iter")]
///
/// This is the mutable counterpart of [`StridedIter`].
///
/// Each call yields an exclusive reference tied to the borrow of
/// the iterator itself. The iterator supports forward and backward
/// traversal.
///
/// Indices follow:
///
/// `index_k = front + k * stride`
///
/// until the inclusive bound `back` is reached.
///
/// # Invariants
/// - `front <= back`, or the iterator is empty.
/// - All generated indices must lie within `storage`.
///
/// # Aliasing
///
/// Although indices may repeat if constructed improperly,
/// the borrow checker prevents simultaneous mutable aliases
/// because each reference is tied to `&mut self`.
///
/// Violating bounds conditions will cause a panic.
/// No unsafe code is used.
pub struct StridedIterMut<'a, T> {
    storage: &'a mut [T],
    core: StridedIterCore,
}
impl<'a, T> StridedIterMut<'a, T> {
    /* contructors */

    /// Creates a mutable strided iterator
    /// from a starting index, a number of elements, and a stride.
    ///
    /// Generates indices:
    /// `start + k * stride` for `k` in `0..remaining`.
    ///
    /// # Panics
    /// Panics if `stride == 0`.
    /// May panic when advanced if the generated indices are out of bounds.
    pub const fn from_count(
        storage: &'a mut [T],
        start: usize,
        remaining: usize,
        stride: usize,
    ) -> Self {
        unwrap![some_map_into_expect NonZeroUsize::new(stride),
            |v| Self::from_count_nz(storage, start, remaining, v), "stride must be > 0"]
    }
    /// Like [`from_count`][Self::from_count] but takes a non-zero stride.
    pub const fn from_count_nz(
        storage: &'a mut [T],
        start: usize,
        remaining: usize,
        stride: NonZeroUsize,
    ) -> Self {
        Self {
            storage,
            core: StridedIterCore::from_count(start, remaining, stride),
        }
    }

    /// Creates a mutable strided iterator
    /// from inclusive front and back limits and a stride.
    ///
    /// Iteration proceeds from `front` toward `back` (inclusive limit)
    /// in steps of `stride`. The iterator is empty if `front > back`.
    ///
    /// # Panics
    /// Panics if `stride == 0`.
    /// May panic when advanced if the generated indices are out of bounds.
    pub const fn from_bounds(
        storage: &'a mut [T],
        front: usize,
        back: usize,
        stride: usize,
    ) -> Self {
        unwrap![some_map_into_expect NonZeroUsize::new(stride),
            |v| Self::from_bounds_nz(storage, front, back, v), "stride must be > 0"]
    }
    /// Like [`from_bounds`][Self::from_bounds] but takes a non-zero `stride`.
    pub const fn from_bounds_nz(
        storage: &'a mut [T],
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

    /// Advances the iterator and returns an exclusive reference to the next value from the front.
    pub const fn next(&mut self) -> Option<&mut T> {
        Some(&mut self.storage[unwrap!(some? self.core.next_front_index())])
    }
    /// Returns an exclusive reference to the next value from the front,
    /// without updating the iterator.
    pub const fn peek(&mut self) -> Option<&mut T> {
        Some(&mut self.storage[unwrap!(some? self.core.peek_next_front_index())])
    }
    /// Advances the iterator and returns an exclusive reference to the next value from the back.
    pub const fn next_back(&mut self) -> Option<&mut T> {
        Some(&mut self.storage[unwrap!(some? self.core.next_back_index())])
    }
    /// Returns an exclusive reference to the next value from the back,
    /// without updating the iterator.
    pub const fn peek_back(&mut self) -> Option<&mut T> {
        Some(&mut self.storage[unwrap!(some? self.core.peek_next_back_index())])
    }
}
impl<'a, T> IteratorLending for StridedIterMut<'a, T> {
    type Item<'b>
        = &'b mut T
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
impl<'a, T> IteratorLendingExactSize for StridedIterMut<'a, T> {}
impl<'a, T> IteratorLendingPeek for StridedIterMut<'a, T> {
    fn peek<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        self.peek()
    }
}
impl<'a, T> IteratorLendingDoubleEnded for StridedIterMut<'a, T> {
    fn next_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        Some(&mut self.storage[self.core.next_back_index()?])
    }
}
impl<'a, T> IteratorLendingPeekDoubleEnded for StridedIterMut<'a, T> {
    fn peek_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        self.peek_back()
    }
}
