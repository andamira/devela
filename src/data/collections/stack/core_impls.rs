// devela::data::impls
//
//!
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    data::{Array, Stack},
    mem::Storage,
};
use core::fmt;

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize> Clone for Stack<T, S, CAP>
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
impl<T: Copy, S: Storage, const CAP: usize> Copy for Stack<T, S, CAP> where S::Stored<[T; CAP]>: Copy
{}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize> fmt::Debug for Stack<T, S, CAP>
where
    S::Stored<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![Stack]);
        debug.field("CAP", &CAP).field("len", &self.len);

        if CAP <= 6 {
            debug.field("nodes", &self.array);
        } else {
            // IMPROVE: show the first 3 and the last 3
            debug.field("array { ... }", &());
        }
        debug.finish()
    }
}

// T:PartialEq
impl<T: PartialEq, S: Storage, const CAP: usize> PartialEq for Stack<T, S, CAP>
where
    S::Stored<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.len == other.len
    }
}
// T:Eq
impl<T: Eq, S: Storage, const CAP: usize> Eq for Stack<T, S, CAP> where S::Stored<[T; CAP]>: Eq {}

// S:() + T:Default
impl<T: Default, const CAP: usize> Default for Stack<T, (), CAP> {
    /// Returns an empty stack, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self {
            array: Array::default(),
            len: 0,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
impl<T: Default, const CAP: usize> Default for Stack<T, Boxed, CAP> {
    /// Returns an empty stack, allocated in the heap,
    /// using the default value to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use devela::data::BoxedStack;
    /// let mut s = BoxedStack::<i32, 100>::default();
    /// ```
    fn default() -> Self {
        Self {
            array: Array::default(),
            len: 0,
        }
    }
}

/* From<IntoIterator<Item = T>> */

impl<T: Default, I, const CAP: usize> From<I> for Stack<T, (), CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a stack filled with an iterator, in the stack.
    /// # Examples
    /// ```
    /// # use devela::data::DirectStack;
    /// let s: DirectStack<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> Stack<T, (), CAP> {
        let mut s = Stack::<T, (), CAP>::default();
        let _ = s.extend(iterator);
        s
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
impl<T: Default, I, const CAP: usize> From<I> for Stack<T, Boxed, CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a stack filled with an iterator, in the heap.
    /// # Examples
    /// ```
    /// # use devela::data::BoxedStack;
    /// let s: BoxedStack<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> Stack<T, Boxed, CAP> {
        let mut s = Stack::<T, Boxed, CAP>::default();
        let _ = s.extend(iterator);
        s
    }
}
