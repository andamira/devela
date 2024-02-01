// devela::data::collections::stack
//
//! Stacks ar linear lists for which all accesses are made from one end.
//!
//! <https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    data::{Array, DataCollection, DataResult as Result},
    mem::Storage,
};

/* types */

/// A stack backed by an [`Array`].
///
/// It implements methods that operate from the back.
///
/// - General methods:
///   - [`new`][Self::new],
/// [`len`][Self::len], [`is_empty`][Self::is_empty], [`is_full`][Self::is_full],
/// [`clear`][Self::clear], [`contains`][Self::contains],
/// [`capacity`][Self::capacity], [`remaining_capacity`][Self::remaining_capacity].
///   - [`iter`][Self::iter],
/// [`extend`][Self::extend],
/// [`from_array`][Self::from_array], [`to_array`][Self::to_array],
/// [`to_vec`][Self::to_vec].
///
/// - Stack methods:
///   - push: [`push`][Self::push].
///   - pop: [`pop`][Self::pop].
///   - peek:
/// [`peek`][Self::peek]*([mut][Self::peek_mut])*,
/// [`peek_nth`][Self::peek_nth]*([mut][Self::peek_nth_mut])*,
///   - drop:
/// [`drop`][Self::drop], [`drop_n`][Self::drop_n],
/// [`drop_replace_default`][Self::drop_replace_default].
///   - swap: [`swap`][Self::swap], [`swap2`][Self::swap2],
///   - rot:
/// [`rot`][Self::rot], [`rot_cc`][Self::rot_cc],
/// [`rot2`][Self::rot2], [`rot2_cc`][Self::rot2_cc].
///   - dup:
/// [`dup`][Self::dup], [`dup2`][Self::dup2].
///   - over:
/// [`over`][Self::over], [`over2`][Self::over2].
///   - tuck:
/// [`tuck`][Self::tuck], [`tuck2`][Self::tuck2].
///
/// See also the related trait: [`DataStack`][crate::DataStack].
pub struct Stack<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
}

/// A [`Stack`] stored in the stack.
pub type DirectStack<T, const CAP: usize> = Stack<T, (), CAP>;

/// A [`Stack`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub type BoxedStack<T, const CAP: usize> = Stack<T, Boxed, CAP>;

/* iterators */

/// A stack iterator.
pub struct StackIter<'s, T, S: Storage, const CAP: usize> {
    pub(super) stack: &'s Stack<T, S, CAP>,
    pub(super) idx: usize,
}

impl<'s, T, S: Storage, const CAP: usize> Iterator for StackIter<'s, T, S, CAP> {
    type Item = &'s T;
    /// Iterates over shared references.
    ///
    /// # Example
    /// ```
    /// # use devela::data::DirectStack;
    /// let s = DirectStack::<i32, 4>::from([1, 2]);
    ///
    /// let mut si = s.iter();
    /// assert_eq![Some(&1), si.next()];
    /// assert_eq![Some(&2), si.next()];
    /// assert_eq![None, si.next()];
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.idx == self.stack.len() {
            None
        } else {
            Some(&self.stack.array[self.idx])
        };
        self.idx += 1;
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.stack.len()))
    }
}

impl<'s, T, S: Storage, const CAP: usize> ExactSizeIterator for StackIter<'s, T, S, CAP> {}