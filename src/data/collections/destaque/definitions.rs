// devela::data::collections::destaque::definitions
//
//! Double-ended queues are linear lists for which any accesses are made from
//! either end.
//

use crate::{
    data::{Array, DataCollection, DataQueue, DataResult as Result},
    mem::Storage,
};

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

/* types */

/// A double-ended queue and stack backed by an [`Array`].
///
/// It has the queue and stack methods implemented for both
/// the front and the back sides.
///
/// [`Queue`]: crate::data::Queue
/// [`Stack`]: crate::data::Stack
// TODO:IMPROVE docs
pub struct Destaque<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
    pub(crate) front: usize,
    pub(crate) back: usize,
}

/// A [`Destaque`] stored in the stack.
pub type DirectDestaque<T, const CAP: usize> = Destaque<T, (), CAP>;

/// A [`Destaque`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub type BoxedDestaque<T, const CAP: usize> = Destaque<T, Boxed, CAP>;

/* iterators */

/// An iterator over [`Destaque`] elements.
pub struct DestaqueIter<'s, T, S: Storage, const CAP: usize> {
    pub(super) deque: &'s Destaque<T, S, CAP>,
    pub(super) idx: usize,
}

impl<'s, T, S: Storage, const CAP: usize> Iterator for DestaqueIter<'s, T, S, CAP> {
    type Item = &'s T;
    /// Iterates over shared references.
    /// # Example
    /// ```
    /// # use devela::data::DirectDestaque;
    /// let mut dq = DirectDestaque::<i32, 4>::from([1, 2]);
    /// dq.pop_front();
    /// dq.push_back(3);
    /// dq.pop_front();
    /// dq.push_back(4);
    ///
    /// let mut dqi = dq.iter();
    /// assert_eq![Some(&3), dqi.next()];
    /// assert_eq![Some(&4), dqi.next()];
    /// assert_eq![None, dqi.next()];
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.idx == self.deque.len() {
            None
        } else {
            Some(&self.deque.array[self.deque.idx_front(self.idx)])
        };
        self.idx += 1;
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.deque.len(), Some(self.deque.len()))
    }
}

impl<'s, T, S: Storage, const CAP: usize> ExactSizeIterator for DestaqueIter<'s, T, S, CAP> {}

impl<'s, T, S: Storage, const CAP: usize> DoubleEndedIterator for DestaqueIter<'s, T, S, CAP> {
    /// Iterates over shared references.
    /// # Example
    /// ```
    /// # use devela::data::DirectDestaque;
    /// let mut dq = DirectDestaque::<i32, 4>::from([1, 2]);
    /// dq.pop_front();
    /// dq.push_back(3);
    /// dq.pop_front();
    /// dq.push_back(4);
    ///
    /// let mut dqi = dq.iter();
    /// assert_eq![Some(&3), dqi.next()];
    /// assert_eq![Some(&4), dqi.next()];
    /// assert_eq![None, dqi.next()];
    ///
    /// let mut dqi = dq.iter();
    /// assert_eq![Some(&4), dqi.next_back()];
    /// assert_eq![Some(&3), dqi.next_back()];
    /// assert_eq![None, dqi.next_back()];
    /// ```
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = if self.idx == self.deque.len() {
            None
        } else {
            Some(&self.deque.array[self.deque.idx_back(self.idx)])
        };
        self.idx += 1;
        item
    }
}
