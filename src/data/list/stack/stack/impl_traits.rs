// devela::data::list::stack::stack::impl_traits
//
//!
//

#[cfg(feature = "alloc")]
use crate::Boxed;
use crate::{
    _core::fmt, Array, Bare, ConstDefault, DataCollection, DataStack, NotAvailable,
    NotEnoughElements, NotEnoughSpace, Ordering, Stack, StackIter, Storage,
};

// helper macro for implementing traits for a Stack depending on the custom index size.
macro_rules! impl_stack {
    () => {
        impl_stack![
            u8:"_stack_u8", u16:"_stack_u16", u32:"_stack_u32", usize:"_stack_usize"];
    };

    // $IDX : the index type. E.g. u8, usize
    // $cap:  the capability feature that enables the given implementation. E.g "_stack_u8".
    ($( $IDX:ty: $cap:literal ),+) => {
        $(
            #[cfg(feature = $cap )]
            impl_stack![@$IDX:$cap];
        )+
    };
    (@$IDX:ty : $cap:literal) => { $crate::paste! {
        /* impl data traits */

        impl<T, const LEN: usize, S: Storage> DataCollection for Stack<T, LEN, $IDX, S> {
            type Element = T;
            fn collection_capacity(&self)
                -> Result<usize, NotAvailable> { Ok(self.capacity() as usize) }
            fn collection_len(&self)
                -> Result<usize, NotAvailable> { Ok(self.len() as usize) }
            fn collection_is_empty(&self)
                -> Result<bool, NotAvailable> { Ok(self.is_empty()) }
            fn collection_is_full(&self)
                -> Result<bool, NotAvailable> { Ok(self.is_full()) }
            fn collection_contains(&self, element: Self::Element)
                -> Result<bool, NotAvailable> where T: PartialEq {
                    Ok(self.contains(&element)) }
            fn collection_count(&self, element: &Self::Element)
                -> Result<usize, NotAvailable> where T: PartialEq {
                    Ok(self.iter().filter(|&e| e == element).count()) }
        }

        // safe alternative with T: Clone
        #[rustfmt::skip]
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, const CAP: usize, S: Storage> DataStack for Stack<T, CAP, $IDX, S> {
            fn stack_pop(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop()
            }
            fn stack_push(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push(element)
            }
        }
        // unsafe alternative without T: Clone
        #[rustfmt::skip]
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, const CAP: usize, S: Storage> DataStack for Stack<T, CAP, $IDX, S> {
            fn stack_pop(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop()
            }
            fn stack_push(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push(element)
            }
        }

        /* impl From<IntoIterator<Item = T>> */

        impl<T: Default, I, const CAP: usize> From<I> for Stack<T, CAP, $IDX, Bare>
        where
            I: IntoIterator<Item = T>,
        {
            /// Returns a stack filled with an iterator, in the stack.
            /// # Examples
            /// ```
            /// # use devela::data::StackU8;
            /// let s: StackU8<_, 3> = [1, 2, 3].into();
            /// ```
            fn from(iterator: I) -> Stack<T, CAP, $IDX, Bare> {
                let mut s = Stack::<T, CAP, $IDX, Bare>::default();
                let _ = s.extend(iterator);
                s
            }
        }

        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        impl<T: Default, I, const CAP: usize> From<I> for Stack<T, CAP, $IDX, Boxed>
        where
            I: IntoIterator<Item = T>,
        {
            /// Returns a stack filled with an iterator, in the heap.
            /// # Examples
            /// ```
            /// # use devela::{Boxed, StackU8};
            /// let s: StackU8<_, 3, Boxed> = [1, 2, 3].into();
            /// ```
            fn from(iterator: I) -> Stack<T, CAP, $IDX, Boxed> {
                let mut s = Stack::<T, CAP, $IDX, Boxed>::default();
                let _ = s.extend(iterator);
                s
            }
        }

        /* impl for StackIter */

        impl<'s, T, const CAP: usize, S: Storage> Iterator for StackIter<'s, T, CAP, $IDX, S> {
            type Item = &'s T;

            /// Iterates over shared references.
            ///
            /// # Example
            /// ```
            /// # use devela::data::StackU8;
            /// let s = StackU8::<i32, 4>::from([1, 2]);
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
                    Some(&self.stack.data[self.idx])
                };
                self.idx += 1;
                item
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.stack.len as usize, Some(self.stack.len as usize))
            }
        }
        impl<'s, T, const CAP: usize, S: Storage> ExactSizeIterator
            for StackIter<'s, T, CAP, $IDX, S> {}

        /* PartialOrd, Ord */

        // T: PartialOrd
        impl<T: PartialOrd, const CAP: usize, S: Storage> PartialOrd for Stack<T, CAP, $IDX, S>
        where
            S::Stored<[T; CAP]>: PartialOrd,
        {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.iter().partial_cmp(other.iter())
            }
        }

        // T: Ord
        impl<T: Ord, const CAP: usize, S: Storage> Ord for Stack<T, CAP, $IDX, S>
        where
            S::Stored<[T; CAP]>: Ord,
        {
            fn cmp(&self, other: &Self) -> Ordering {
                self.iter().cmp(other.iter())
            }
        }
    }};
}
impl_stack!();

// T: Clone
impl<T: Clone, const CAP: usize, IDX: Copy, S: Storage> Clone for Stack<T, CAP, IDX, S>
where
    S::Stored<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), len: self.len }
    }
}

// T: Copy
impl<T: Copy, const CAP: usize, IDX: Copy, S: Storage> Copy for Stack<T, CAP, IDX, S> where
    S::Stored<[T; CAP]>: Copy
{
}

// T: Debug
impl<T: fmt::Debug, const CAP: usize, IDX: fmt::Debug, S: Storage> fmt::Debug
    for Stack<T, CAP, IDX, S>
where
    S::Stored<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stack")
            .field("CAP", &CAP)
            .field("len", &self.len)
            .field("data", &self.data)
            .finish()
    }
}

// T: PartialEq
impl<T: PartialEq, const CAP: usize, IDX: PartialEq, S: Storage> PartialEq for Stack<T, CAP, IDX, S>
where
    S::Stored<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.len == other.len
    }
}
// T: Eq
impl<T: Eq, const CAP: usize, IDX: Eq, S: Storage> Eq for Stack<T, CAP, IDX, S> where
    S::Stored<[T; CAP]>: Eq
{
}

// T: Default, S: Bare
impl<T: Default, const CAP: usize, IDX: Default> Default for Stack<T, CAP, IDX, Bare> {
    /// Returns an empty stack, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self { data: Array::default(), len: IDX::default() }
    }
}

// T: ConstDefault, S: Bare
impl<T: ConstDefault, const CAP: usize, IDX: ConstDefault> ConstDefault
    for Stack<T, CAP, IDX, Bare>
{
    /// Returns an empty stack, allocated in the stack,
    /// using the default value to fill the remaining free data.
    const DEFAULT: Self = Self { data: Array::DEFAULT, len: IDX::DEFAULT };
}

// T: Default, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<T: Default, const CAP: usize, IDX: Default> Default for Stack<T, CAP, IDX, Boxed> {
    /// Returns an empty stack, allocated in the heap,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self { data: Array::default(), len: IDX::default() }
    }
}
