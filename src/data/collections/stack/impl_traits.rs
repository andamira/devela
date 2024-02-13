// devela::data::impl_traits
//
//!
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    code::ConstDefault,
    data::{Array, DataCollection, DataResult as Result, DataStack, Stack, StackIter},
    mem::{Bare, Storage},
};
use core::fmt;

// helper macro for implementing traits for a Stack depending on the custom index size.
macro_rules! impl_stack {
    () => {
        impl_stack![Stack, u8];
        impl_stack![Stack, u16];
        impl_stack![Stack, u32];
        impl_stack![Stack, usize];
    };

    // $name : name prefix. E.g.: Stack8b
    // $IDX : the index type. E.g. u8, usize
    ( $name:ident, $IDX:ty ) => { crate::code::paste! {

        /* impl data traits */

        impl<T, S: Storage, const LEN: usize> DataCollection for Stack<T, S, LEN, $IDX> {
            type Element = T;
            fn collection_capacity(&self) -> Result<usize> { Ok(self.capacity() as usize) }
            fn collection_len(&self) -> Result<usize> { Ok(self.len() as usize) }
            fn collection_is_empty(&self) -> Result<bool> { Ok(self.is_empty()) }
            fn collection_is_full(&self) -> Result<bool> { Ok(self.is_full()) }
            fn collection_contains(&self, element: Self::Element) -> Result<bool> where T: PartialEq {
                Ok(self.contains(&element))
            }
            fn collection_count(&self, element: &Self::Element) -> Result<usize> where T: PartialEq {
                Ok(self.iter().filter(|&e| e == element).count())
            }
        }

        // safe alternative with T: Clone
        #[rustfmt::skip]
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, S: Storage, const CAP: usize> DataStack for Stack<T, S, CAP, $IDX> {
            fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop()
            }
            fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push(element)
            }
        }
        // unsafe alternative without T: Clone
        #[rustfmt::skip]
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, S: Storage, const CAP: usize> DataStack for Stack<T, S, CAP, $IDX> {
            fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop()
            }
            fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push(element)
            }
        }

        /* impl From<IntoIterator<Item = T>> */

        impl<T: Default, I, const CAP: usize> From<I> for Stack<T, Bare, CAP, $IDX>
        where
            I: IntoIterator<Item = T>,
        {
            /// Returns a stack filled with an iterator, in the stack.
            /// # Examples
            /// ```
            /// # use devela::data::StackU8;
            /// let s: StackU8<_, (), 3> = [1, 2, 3].into();
            /// ```
            fn from(iterator: I) -> Stack<T, Bare, CAP, $IDX> {
                let mut s = Stack::<T, Bare, CAP, $IDX>::default();
                let _ = s.extend(iterator);
                s
            }
        }

        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        impl<T: Default, I, const CAP: usize> From<I> for Stack<T, Boxed, CAP, $IDX>
        where
            I: IntoIterator<Item = T>,
        {
            /// Returns a stack filled with an iterator, in the heap.
            /// # Examples
            /// ```
            /// # use devela::all::{Boxed, StackU32};
            /// let s: StackU32<_, Boxed, 3> = [1, 2, 3].into();
            /// ```
            fn from(iterator: I) -> Stack<T, Boxed, CAP, $IDX> {
                let mut s = Stack::<T, Boxed, CAP, $IDX>::default();
                let _ = s.extend(iterator);
                s
            }
        }

        /* impl for StackIter */

        impl<'s, T, S: Storage, const CAP: usize> Iterator for StackIter<'s, T, S, CAP, $IDX> {
            type Item = &'s T;

            /// Iterates over shared references.
            ///
            /// # Example
            /// ```
            /// # use devela::data::StackU8;
            /// let s = StackU8::<i32, (), 4>::from([1, 2]);
            ///
            /// let mut si = s.iter();
            /// assert_eq![Some(&1), si.next()];
            /// assert_eq![Some(&2), si.next()];
            /// assert_eq![None, si.next()];
            /// ```
            fn next(&mut self) -> Option<Self::Item> {
                let item = if self.idx == self.stack.len as usize {
                    None
                } else {
                    Some(&self.stack.array[self.idx])
                };
                self.idx += 1;
                item
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.stack.len as usize, Some(self.stack.len as usize))
            }
        }
        impl<'s, T, S: Storage, const CAP: usize> ExactSizeIterator
            for StackIter<'s, T, S, CAP, $IDX> {}

    }};
}
impl_stack!();

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
