// devela_base_core::data::access::iter::strided
//
//! Defines [`StridedIter`], [`StridedIterMut`].
//

use crate::{IteratorLending, IteratorLendingExactSize, IteratorLendingPeek, is, unwrap};

/// Core state for strided iteration over a contiguous storage.
///
/// Maintains the current index, remaining element count,
/// and fixed stride between successive elements.
struct StridedIterCore {
    current: usize,
    remaining: usize,
    stride: usize,
}
impl StridedIterCore {
    /// Creates a new strided iteration state.
    const fn new(current: usize, remaining: usize, stride: usize) -> Self {
        Self { current, remaining, stride }
    }
    /// Advances the iterator and returns the next index in the iteration sequence.
    const fn next_index(&mut self) -> Option<usize> {
        if self.remaining == 0 {
            None
        } else {
            let idx = self.current;
            self.current += self.stride;
            self.remaining -= 1;
            Some(idx)
        }
    }
    /// Returns the next index in the iteration sequence without updating the iterator.
    const fn peek_next_index(&self) -> Option<usize> {
        is![self.remaining == 0; None; Some(self.current)]
    }
}

/// Iterates over a slice using an affine index progression.
///
/// Each successive element is accessed at:
/// `index_k = start + k * stride` for `k` in `0..remaining`.
///
/// This iterator does not assume any dimensional structure.
/// It can be used to:
/// - Traverse rows, columns, or diagonals of a 2D layout.
/// - Project channels from interleaved buffers (e.g. RGBRGB…).
/// - Downsample a sequence by stepping every `stride` elements.
/// - Traverse a collapsed axis of an n-dimensional layout.
///
/// # Invariants
///
/// The iterator assumes that:
/// - `remaining == 0`, or
/// - `start + stride * (remaining - 1) < storage.len()`.
///
/// Violating this condition will cause a panic during iteration.
pub struct StridedIter<'a, T> {
    storage: &'a [T],
    core: StridedIterCore,
}
impl<'a, T> StridedIter<'a, T> {
    /// Creates a new shared strided iterator.
    pub const fn new(storage: &'a [T], current: usize, remaining: usize, stride: usize) -> Self {
        Self {
            storage,
            core: StridedIterCore::new(current, remaining, stride),
        }
    }
    /// Advances the iterator and returns a shared reference to the next value.
    pub const fn next(&mut self) -> Option<&T> {
        Some(&self.storage[unwrap!(some? self.core.next_index())])
    }
    /// Returns a shared reference to the next value, without updating the iterator.
    pub const fn peek(&self) -> Option<&T> {
        Some(&self.storage[unwrap!(some? self.core.peek_next_index())])
    }
}
impl<'a, T> Iterator for StridedIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(&self.storage[self.core.next_index()?])
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.core.remaining, Some(self.core.remaining))
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
        (self.core.remaining, Some(self.core.remaining))
    }
}
impl<'a, T> IteratorLendingExactSize for StridedIter<'a, T> {}
impl<'a, T> IteratorLendingPeek for StridedIter<'a, T> {
    fn peek<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        Self::peek(self)
    }
}

/// Lending iterator over mutable references using an affine index progression.
///
/// Each call yields an exclusive reference tied to the borrow of the iterator.
///
/// This is the mutable counterpart of [`StridedIter`].
///
/// Successive indices follow: `index_k = start + k * stride`
///
/// The same bounds invariants apply.
///
/// # Aliasing
///
/// Although the iterator may revisit the same index (e.g. `stride == 0`),
/// the borrow checker prevents simultaneous mutable aliases because each
/// reference is tied to `&mut self`.
pub struct StridedIterMut<'a, T> {
    storage: &'a mut [T],
    core: StridedIterCore,
}
impl<'a, T> StridedIterMut<'a, T> {
    /// Creates a new mutable strided iterator.
    pub const fn new(
        storage: &'a mut [T],
        current: usize,
        remaining: usize,
        stride: usize,
    ) -> Self {
        Self {
            storage,
            core: StridedIterCore::new(current, remaining, stride),
        }
    }
    /// Advances the iterator and returns an exclusive reference to the next value.
    pub const fn next(&mut self) -> Option<&mut T> {
        Some(&mut self.storage[unwrap!(some? self.core.next_index())])
    }
    /// Returns an exclusive reference to the next value without updating the iterator.
    pub const fn peek(&mut self) -> Option<&mut T> {
        Some(&mut self.storage[unwrap!(some? self.core.peek_next_index())])
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
        (self.core.remaining, Some(self.core.remaining))
    }
}
impl<'a, T> IteratorLendingExactSize for StridedIterMut<'a, T> {}
impl<'a, T> IteratorLendingPeek for StridedIterMut<'a, T> {
    fn peek<'b>(&'b mut self) -> Option<Self::Item<'b>> {
        self.peek()
    }
}
