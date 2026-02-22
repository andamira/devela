// devela_base_core::data::access::iter::strided
//
//! Defines ([`StridedIterCore`]).
//

use crate::{NonZeroUsize, is};

#[doc = crate::_tags!(iterator)]
/// Core state for strided iteration over a contiguous storage.
#[doc = crate::_doc_location!("data/access/iter")]
///
/// Maintains the front and back indices, and fixed stride between successive elements.
pub(crate) struct StridedIterCore {
    front: usize,
    back: usize,
    stride: NonZeroUsize,
}
impl StridedIterCore {
    /// Creates a strided iterator state
    /// from a starting index, a number of elements, and a non-zero stride.
    ///
    /// # Panics
    /// In debug builds, panics in case of overflow.
    pub(crate) const fn from_count(start: usize, remaining: usize, stride: NonZeroUsize) -> Self {
        debug_assert! {
            remaining == 0 || match stride.get().checked_mul(remaining - 1) {
                Some(off) => match start.checked_add(off) {
                    Some(_) => true,
                    None => false,
                },
                None => false,
            }
        }
        let back = is![remaining == 0; start; start + stride.get() * (remaining - 1)];
        Self { front: start, back, stride }
    }
    /// Creates strided iteration state
    /// from an inclusive front/back range aligned to `stride` steps.
    ///
    /// # Panics
    /// In debug builds, panics in case of misalign.
    pub(crate) const fn from_bounds(front: usize, back: usize, stride: NonZeroUsize) -> Self {
        debug_assert!(
            front > back || (back - front) % stride.get() == 0,
            "bounds must be stride-aligned"
        );
        Self { front, back, stride }
    }

    /// Returns the number of elements remaining in the iterator.
    pub(crate) const fn len(&self) -> usize {
        if self.front > self.back {
            0
        } else {
            ((self.back - self.front) / self.stride.get()) + 1
        }
    }

    /// Advances the iterator and returns the next index from the front.
    pub(crate) const fn next_front_index(&mut self) -> Option<usize> {
        // if unlikely(self.front > self.back) { // MAYBE:BENCH
        if self.front > self.back {
            None
        } else {
            let idx = self.front;
            // self.front += self.stride.get();
            // NOTE: for the last element, forces an empty sentinel state
            // if unlikely(self.front == self.back) { // MAYBE:BENCH
            if self.front == self.back {
                self.front = 1;
                self.back = 0;
            } else {
                self.front += self.stride.get();
            }
            Some(idx)
        }
    }
    /// Advances the iterator and returns the next index from the back.
    pub(crate) const fn next_back_index(&mut self) -> Option<usize> {
        if self.front > self.back {
            None
        } else {
            let idx = self.back;
            // self.back -= self.stride.get();
            // NOTE: for the last element, forces an empty sentinel state
            if self.front == self.back {
                self.front = 1;
                self.back = 0;
            } else {
                self.back -= self.stride.get();
            }
            Some(idx)
        }
    }
    /// Returns the next index from the front without updating the iterator.
    pub(crate) const fn peek_next_front_index(&self) -> Option<usize> {
        is![self.front > self.back; None; Some(self.front)]
    }
    /// Returns the next index from the back without updating the iterator.
    pub(crate) const fn peek_next_back_index(&self) -> Option<usize> {
        is![self.front > self.back; None; Some(self.back)]
    }
}
