// devela::data::impls
//
//!
//

use super::StackIter;
#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    code::ConstDefault,
    data::{Array, Stack},
    mem::{Bare, Storage},
};
use core::fmt;

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize, IDX: Copy> Clone for Stack<T, S, CAP, IDX>
where
    S::Stored<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
            len: self.len,
        }
    }
}

// T:Copy
impl<T: Copy, S: Storage, const CAP: usize, IDX: Copy> Copy for Stack<T, S, CAP, IDX> where
    S::Stored<[T; CAP]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize, IDX: fmt::Debug> fmt::Debug
    for Stack<T, S, CAP, IDX>
where
    S::Stored<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![Stack]);
        debug.field("CAP", &CAP).field("len", &self.len);

        if CAP <= 6 {
            debug.field("array", &self.array);
        } else {
            // IMPROVE: show the first 3 and the last 3
            debug.field("array { ... }", &());
        }
        debug.finish()
    }
}

// T:PartialEq
impl<T: PartialEq, S: Storage, const CAP: usize, IDX: PartialEq> PartialEq for Stack<T, S, CAP, IDX>
where
    S::Stored<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.len == other.len
    }
}
// T:Eq
impl<T: Eq, S: Storage, const CAP: usize, IDX: Eq> Eq for Stack<T, S, CAP, IDX> where
    S::Stored<[T; CAP]>: Eq
{
}

// S:Bare + T:Default
impl<T: Default, const CAP: usize, IDX: Default> Default for Stack<T, Bare, CAP, IDX> {
    /// Returns an empty stack, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self {
            array: Array::default(),
            len: IDX::default(),
        }
    }
}

// S:Bare + T:ConstDefault
impl<T: ConstDefault, const CAP: usize, IDX: ConstDefault> ConstDefault
    for Stack<T, Bare, CAP, IDX>
{
    /// Returns an empty stack, allocated in the stack,
    /// using the default value to fill the remaining free data.
    const DEFAULT: Self = Self {
        array: Array::DEFAULT,
        len: IDX::DEFAULT,
    };
}

// S:Boxed + T:Default
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
impl<T: Default, const CAP: usize, IDX: Default> Default for Stack<T, Boxed, CAP, IDX> {
    /// Returns an empty stack, allocated in the heap,
    /// using the default value to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use devela::all::{Boxed, StackU32};
    /// let mut s = StackU32::<i32, Boxed, 100>::default();
    /// ```
    fn default() -> Self {
        Self {
            array: Array::default(),
            len: IDX::default(),
        }
    }
}
