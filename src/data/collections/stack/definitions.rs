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
/// All the methods operate from the back.
///
/// See also the related trait: [`DataStack`][crate::DataStack].
///
/// ## Methods
/// - Constructors:
///   [`new`][Self::new],
///   [`new_copied`][Self::new_copied],
///   [`from_array`][Self::from_array].
/// - Deconstructors:
///   [`to_array`][Self::to_array],
///   [`to_vec`][Self::to_vec]*(`alloc`)*.
/// - Queries:
///   [`len`][Self::len], [`is_empty`][Self::is_empty], [`is_full`][Self::is_full],
///   [`capacity`][Self::capacity], [`remaining_capacity`][Self::remaining_capacity],
///   [`contains`][Self::contains].
/// - Iterator related:
///   [`iter`][Self::iter],
///   [`extend`][Self::extend],
///
/// - Stack operations:
///   - clear: [`clear`][Self::clear].
///   - push: [`push`][Self::push].
///   - pop: [`pop`][Self::pop] *(`unsafe_ptr`)*.
///   - peek:
///     [`peek`][Self::peek]*([mut][Self::peek_mut])*,
///     [`peek_nth`][Self::peek_nth]*([mut][Self::peek_nth_mut])*,
///   - drop:
///     [`drop`][Self::drop], [`drop_n`][Self::drop_n],
///     [`drop_replace_default`][Self::drop_replace_default].
///   - nip: [`nip`][Self::nip], [`nip2`][Self::nip2].
///   - swap: [`swap`][Self::swap], [`swap2`][Self::swap2],
///   - rot:
///     [`rot`][Self::rot], [`rot_cc`][Self::rot_cc],
///     [`rot2`][Self::rot2], [`rot2_cc`][Self::rot2_cc].
///
/// - Stack [operations that depend on `Clone`](#operations-that-depend-on-clone).
///   - pop: [`pop`][Self::pop] *(safe)*.
///   - dup: [`dup`][Self::dup], [`dup2`][Self::dup2].
///   - over: [`over`][Self::over], [`over2`][Self::over2].
///   - tuck: [`tuck`][Self::tuck], [`tuck2`][Self::tuck2].
///
/// - Stack [chainable *const* operations](#chainable-const-operations) for `T: Copy`.
///   - clear: [`own_clear`][Self::own_clear].
///   - push: [`own_push`][Self::push]*([uc][Self::own_push_unchecked])*,
///   - pop: [`own_pop`][Self::pop]*([uc][Self::own_pop_unchecked])*.
///   - drop:
///     [`own_drop`][Self::own_drop]*([uc][Self::own_drop_unchecked])*,
///     [`own_drop_n`][Self::own_drop_n]*([uc][Self::own_drop_n_unchecked])*.
///   - nip:
///     [`own_nip`][Self::own_nip]*([uc][Self::own_nip_unchecked])*,
///     [`own_nip2`][Self::own_nip2]*([uc][Self::own_nip2_unchecked])*.
///   - swap:
///     [`own_swap`][Self::own_swap]*([uc][Self::own_swap_unchecked])*,
///     [`own_swap2`][Self::own_swap2]*([uc][Self::own_swap2_unchecked])*.
///   - rot:
///     [`own_rot`][Self::own_rot]*([uc][Self::own_rot_unchecked])*,
///     [`own_rot_cc`][Self::own_rot_cc]*([uc][Self::own_rot_cc_unchecked])*,
///     [`own_rot2`][Self::own_rot2]*([uc][Self::own_rot2_unchecked])*,
///     [`own_rot2_cc`][Self::own_rot2_cc]*([uc][Self::own_rot2_cc_unchecked])*.
///   - dup:
///     [`own_dup`][Self::own_dup]*([uc][Self::own_dup_unchecked])*,
///     [`own_dup2`][Self::own_dup2]*([uc][Self::own_dup2_unchecked])*.
///   - over:
///     [`own_over`][Self::own_over]*([uc][Self::own_over_unchecked])*,
///     [`own_over2`][Self::own_over2]*([uc][Self::own_over2_unchecked])*.
///   - tuck:
///     [`own_tuck`][Self::own_tuck]*([uc][Self::own_tuck_unchecked])*,
///     [`own_tuck2`][Self::own_tuck2]*([uc][Self::own_tuck2_unchecked])*.
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
