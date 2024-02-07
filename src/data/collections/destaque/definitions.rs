// devela::data::collections::destaque::definitions
//
//! Double-ended queues are linear lists for which any accesses are made from
//! either end.
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    data::{Array, DataCollection, DataResult as Result},
    mem::{Bare, Storage},
};

/* types */

/// A double-ended queue and stack backed by an [`Array`].
///
/// It implements methods that operate from both the front and the back.
/// Rememeber that a single-ended *stack* operates only from the back, while a
/// single-ended *queue* pushes to the back and pops from the front.
///
/// See also the related traits:
/// [`DataQueue`][crate::DataQueue], [`DataDeque`][crate::DataDeque],
/// [`DataStack`][crate::DataStack], [`DataDestack`][crate::DataDestack]<br/>
/// and aliases: [`BareDestaque`] and [`BoxedDestaque`].
///
/// ## Methods
/// - General methods:
///   - [`new`][Self::new],
/// [`len`][Self::len], [`is_empty`][Self::is_empty], [`is_full`][Self::is_full],
/// [`clear`][Self::clear], [`contains`][Self::contains],
/// [`capacity`][Self::capacity], [`remaining_capacity`][Self::remaining_capacity].
///   - [`iter`][Self::iter],
/// [`extend_back`][Self::extend_back], [`extend_front`][Self::extend_front],
/// [`from_array`][Self::from_array], [`to_array`][Self::to_array],
/// [`to_vec`][Self::to_vec].
///
/// - Queue and stack methods:
///   - push:
/// [`push_back`][Self::push_back]*([uc][Self::push_back_unchecked])*
///   **=** [`enqueue`][Self::enqueue],
/// [`push_front`][Self::push_front]*([uc][Self::push_front_unchecked])*.
///   - pop:
/// [`pop_front`][Self::pop_front]
///   **=** [`dequeue`][Self::dequeue],
/// [`pop_back`][Self::pop_back].
///   - peek:
/// [`peek_back`][Self::peek_back]*([mut][Self::peek_back_mut])*
/// [`peek_nth_back`][Self::peek_nth_back]*([mut][Self::peek_nth_back_mut])*,
/// [`peek_front`][Self::peek_front]*([mut][Self::peek_front_mut])*,
/// [`peek_nth_front`][Self::peek_nth_front]*([mut][Self::peek_nth_front_mut])*.
///   - drop:
/// [`drop_back`][Self::drop_back],
/// [`drop_front`][Self::drop_front],
/// [`drop_n_back`][Self::drop_n_back],
/// [`drop_n_front`][Self::drop_n_front].
///   - swap:
/// [`swap_back`][Self::swap_back]*([uc][Self::swap_back_unchecked])*,
/// [`swap_front`][Self::swap_front]*([uc][Self::swap_front_unchecked])*,
/// [`swap2_back`][Self::swap2_back]*([uc][Self::swap2_back_unchecked])*,
/// [`swap2_front`][Self::swap2_front]*([uc][Self::swap2_front_unchecked])*,
/// [`swap_ends`][Self::swap_ends], [`swap2_ends`][Self::swap2_ends].
///   - rot:
/// [`rot_right`][Self::rot_right],
/// [`rot_right_n`][Self::rot_right_n],
/// [`rot_left`][Self::rot_left],
/// [`rot_left_n`][Self::rot_left_n].
///   - dup:
/// [`dup_back`][Self::dup_back],
/// [`dup_front`][Self::dup_front],
/// [`dup2_back`][Self::dup2_back],
/// [`dup2_front`][Self::dup2_front].
///   - over:
/// [`over_back`][Self::over_back],
/// [`over_front`][Self::over_front],
/// [`over2_back`][Self::over2_back],
/// [`over2_front`][Self::over2_front].
///   - tuck:
/// [`tuck_back`][Self::tuck_back],
/// [`tuck_front`][Self::tuck_front],
/// [`tuck2_back`][Self::tuck2_back],
/// [`tuck2_front`][Self::tuck2_front].
pub struct Destaque<T, S: Storage, const CAP: usize> {
    pub(super) array: Array<T, S, CAP>,
    pub(super) len: usize,
    pub(super) front: usize,
    pub(super) back: usize,
}

/// A [`Destaque`] stored in the stack.
pub type BareDestaque<T, const CAP: usize> = Destaque<T, Bare, CAP>;

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
    /// # use devela::data::Destaque;
    /// let mut dq = Destaque::<i32, (), 4>::from([1, 2]);
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
    /// # use devela::data::Destaque;
    /// let mut dq = Destaque::<i32, (), 4>::from([1, 2]);
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
