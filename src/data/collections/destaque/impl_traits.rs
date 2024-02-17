// devela::data::collections::destaque::methods
//
//!
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    code::ConstDefault,
    data::{
        Array, DataCollection, DataDeque, DataDesta, DataQueue, DataResult as Result, DataStack,
        Destaque, DestaqueIter,
    },
    mem::{Bare, Storage},
};
use core::{cmp::Ordering, fmt};

// helper macro for implementing traits for a Stack depending on the custom index size.
macro_rules! impl_destaque {
    () => {
        impl_destaque![u8];
        impl_destaque![u16];
        impl_destaque![u32];
        impl_destaque![usize];
    };

    // $IDX : the index type. E.g. u8, usize
    ( $IDX:ty ) => { crate::code::paste! {

        /* impl data traits */

        /* collection */

        #[rustfmt::skip]
        impl<T, S: Storage, const LEN: usize> DataCollection for Destaque<T, S, LEN, $IDX> {
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

        /* queue, deque */

        // safe alternative with T: Clone
        #[rustfmt::skip]
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, S: Storage, const CAP: usize> DataQueue for crate::data::collections::Destaque<T, S, CAP, $IDX> {
            fn queue_pop(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop_front()
            }
            fn queue_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push_back(element)
            }
        }
        #[rustfmt::skip]
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, S: Storage, const CAP: usize> DataDeque for crate::data::collections::Destaque<T, S, CAP, $IDX> {
            fn queue_pop_back(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop_back()
            }
            fn queue_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push_front(element)
            }
        }
        // unsafe alternative without T: Clone
        #[rustfmt::skip]
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, S: Storage, const CAP: usize> DataQueue for crate::data::collections::Destaque<T, S, CAP, $IDX> {
            fn queue_pop(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop_front()
            }
            fn queue_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push_back(element)
            }
        }
        #[rustfmt::skip]
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, S: Storage, const CAP: usize> DataDeque for crate::data::collections::Destaque<T, S, CAP, $IDX> {
            fn queue_pop_back(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop_back()
            }
            fn queue_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push_front(element)
            }
        }

        /* stack, desta */

        // safe alternative with T: Clone
        #[rustfmt::skip]
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, S: Storage, const CAP: usize> DataStack for Destaque<T, S, CAP, $IDX> {
            fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop_back()
            }
            fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push_back(element)
            }
        }
        #[rustfmt::skip]
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, S: Storage, const CAP: usize> DataDesta for Destaque<T, S, CAP, $IDX> {
            fn stack_pop_front(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop_front()
            }
            fn stack_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push_front(element)
            }
        }
        // unsafe alternative without T: Clone
        #[rustfmt::skip]
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, S: Storage, const CAP: usize> DataStack for Destaque<T, S, CAP, $IDX> {
            fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop_back()
            }
            fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push_back(element)
            }
        }
        #[rustfmt::skip]
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, S: Storage, const CAP: usize> DataDesta for Destaque<T, S, CAP, $IDX> {
            fn stack_pop_front(&mut self) -> Result<<Self as DataCollection>::Element> {
                self.pop_front()
            }
            fn stack_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
                self.push_front(element)
            }
        }
        /* impl From<IntoIterator<Item = T>> */

        impl<T: Default, I, const CAP: usize> From<I> for Destaque<T, Bare, CAP, $IDX>
        where
            I: IntoIterator<Item = T>,
        {
            /// Returns a queue filled with an iterator, in the stack.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let q: Destaque" $IDX:camel "<_, (), 3> = [1, 2, 3].into();"]
            /// ```
            fn from(iterator: I) -> Destaque<T, Bare, CAP, $IDX> {
                let mut q = Destaque::<T, Bare, CAP, $IDX>::default();
                let _ = q.extend_back(iterator);
                q
            }
        }

        #[cfg(feature = "alloc")]
        impl<T: Default, I, const CAP: usize> From<I> for Destaque<T, Boxed, CAP, $IDX>
        where
            I: IntoIterator<Item = T>,
        {
            /// Returns a queue filled with an iterator, in the heap.
            /// # Examples
            /// ```
            #[doc = "# use devela::all::{Boxed, Destaque" $IDX:camel "};"]
            #[doc = "let q: Destaque" $IDX:camel "<_, Boxed, 3> = [1, 2, 3].into();"]
            /// ```
            fn from(iterator: I) -> Destaque<T, Boxed, CAP, $IDX> {
                let mut q = Destaque::<T, Boxed, CAP, $IDX>::default();
                let _ = q.extend_back(iterator);
                q
            }
        }

        /* iterators */

        impl<'s, T, S: Storage, const CAP: usize> Iterator for DestaqueIter<'s, T, S, CAP, $IDX> {
            type Item = &'s T;
            /// Iterates over shared references.
            /// # Example
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, (), 4>::from([1, 2]);"]
            /// q.pop_front();
            /// q.push_back(3);
            /// q.pop_front();
            /// q.push_back(4);
            ///
            /// let mut qi = q.iter();
            /// assert_eq![Some(&3), qi.next()];
            /// assert_eq![Some(&4), qi.next()];
            /// assert_eq![None, qi.next()];
            /// ```
            fn next(&mut self) -> Option<Self::Item> {
                let item = if self.idx == self.destaque.len() as usize {
                    None
                } else {
                    Some(&self.destaque.array[self.destaque.idx_front(self.idx as $IDX)])
                };
                self.idx += 1;
                item
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.destaque.len() as usize, Some(self.destaque.len() as usize))
            }
        }
        impl<'s, T, S: Storage, const CAP: usize> ExactSizeIterator for DestaqueIter<'s, T, S, CAP, $IDX> {}

        impl<'s, T, S: Storage, const CAP: usize> DoubleEndedIterator for DestaqueIter<'s, T, S, CAP, $IDX> {
            /// Iterates over shared references.
            /// # Example
            /// ```
            #[doc = "# use devela::all::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, (), 4>::from([1, 2]);"]
            /// q.pop_front();
            /// q.push_back(3);
            /// q.pop_front();
            /// q.push_back(4);
            ///
            /// let mut qi = q.iter();
            /// assert_eq![Some(&3), qi.next()];
            /// assert_eq![Some(&4), qi.next()];
            /// assert_eq![None, qi.next()];
            ///
            /// let mut qi = q.iter();
            /// assert_eq![Some(&4), qi.next_back()];
            /// assert_eq![Some(&3), qi.next_back()];
            /// assert_eq![None, qi.next_back()];
            /// ```
            fn next_back(&mut self) -> Option<Self::Item> {
                let item = if self.idx == self.destaque.len() as usize {
                    None
                } else {
                    Some(&self.destaque.array[self.destaque.idx_back(self.idx as $IDX)])
                };
                self.idx += 1;
                item
            }
        }

        /* PartialOrd, Ord */

        // T:PartialOrd
        impl<T: PartialOrd, S: Storage, const CAP: usize> PartialOrd for Destaque<T, S, CAP, $IDX>
        where
            S::Stored<[T; CAP]>: PartialOrd,
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.iter().partial_cmp(other.iter())
            }
        }

        // T:Ord
        impl<T: Ord, S: Storage, const CAP: usize> Ord for Destaque<T, S, CAP, $IDX>
        where
            S::Stored<[T; CAP]>: Ord,
        {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.iter().cmp(other.iter())
            }
        }
    }};
}
impl_destaque!();

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize, IDX: Clone> Clone for Destaque<T, S, CAP, IDX>
where
    S::Stored<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
            front: self.front.clone(),
            back: self.back.clone(),
            len: self.len.clone(),
        }
    }
}

// T:Copy
impl<T: Copy, S: Storage, const CAP: usize, IDX: Copy> Copy for Destaque<T, S, CAP, IDX> where
    S::Stored<[T; CAP]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize, IDX: fmt::Debug> fmt::Debug
    for Destaque<T, S, CAP, IDX>
where
    S::Stored<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![Destaque]);
        debug
            .field("CAP", &CAP)
            .field("len", &self.len)
            .field("front", &self.front)
            .field("back", &self.back);

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
impl<T: PartialEq, S: Storage, const CAP: usize, IDX: PartialEq> PartialEq
    for Destaque<T, S, CAP, IDX>
where
    S::Stored<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
            && self.len == other.len
            && self.front == other.front
            && self.back == other.back
    }
}
// T:Eq
impl<T: Eq, S: Storage, const CAP: usize, IDX: Eq> Eq for Destaque<T, S, CAP, IDX> where
    S::Stored<[T; CAP]>: Eq
{
}

// S:Bare + T:Default
impl<T: Default, const CAP: usize, IDX: Default> Default for Destaque<T, Bare, CAP, IDX> {
    /// Returns an empty queue, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self {
            array: Array::default(),
            front: IDX::default(),
            back: IDX::default(),
            len: IDX::default(),
        }
    }
}

// S:Bare + T:ConstDefault
impl<T: ConstDefault, const CAP: usize, IDX: ConstDefault> ConstDefault
    for Destaque<T, Bare, CAP, IDX>
{
    /// Returns an empty stack, allocated in the stack,
    /// using the default value to fill the remaining free data.
    const DEFAULT: Self = Self {
        array: Array::DEFAULT,
        front: IDX::DEFAULT,
        back: IDX::DEFAULT,
        len: IDX::DEFAULT,
    };
}

// S:Boxed + T:Default
#[cfg(feature = "alloc")]
impl<T: Default, const CAP: usize, IDX: Default> Default for Destaque<T, Boxed, CAP, IDX> {
    /// Returns an empty queue, allocated in the heap,
    /// using the default value to fill the remaining free data.
    /// # Examples
    /// ```
    /// # use devela::all::{Boxed, DestaqueU8};
    /// let mut q = DestaqueU8::<i32, Boxed, 100>::default();
    /// ```
    fn default() -> Self {
        Self {
            array: Array::default(),
            front: IDX::default(),
            back: IDX::default(),
            len: IDX::default(),
        }
    }
}
