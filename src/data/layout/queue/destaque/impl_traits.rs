// devela::data::layout::queue::destaque::impl_traits
//
//!
//

#[cfg(feature = "alloc")]
use crate::Boxed;
use crate::{
    Array, Bare, ConstInit, DataCollection, DataDeque, DataDesta, DataQueue, DataStack, Destaque,
    DestaqueIter, NotAvailable, NotEnoughElements, NotEnoughSpace, Ordering, Storage,
};
use ::core::fmt;

// helper macro for implementing traits for a Stack depending on the custom index size.
macro_rules! impl_destaque {
    () => {
        impl_destaque![
            u8:"_destaque_u8", u16:"_destaque_u16", u32:"_destaque_u32", usize:"_destaque_usize"];
    };

    // $IDX : the index type. E.g. u8, usize
    // $cap:  the capability feature that enables the given implementation. E.g "_destaque_u8".
    ($( $IDX:ty: $cap:literal ),+) => {
        $(
            #[cfg(feature = $cap )]
            impl_destaque![@$IDX:$cap];
        )+
    };
    (@$IDX:ty : $cap:literal) => { $crate::paste! {
        /* impl data traits */

        /* collection */

        #[rustfmt::skip]
        impl<T, const LEN: usize, S: Storage> DataCollection for Destaque<T, LEN, $IDX, S> {
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

        /* queue, deque */

        // safe alternative with T: Clone
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, const CAP: usize, S: Storage> DataQueue
            for crate::Destaque<T, CAP, $IDX, S> {
            fn queue_pop(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop_front()
            }
            fn queue_push(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push_back(element)
            }
        }
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, const CAP: usize, S: Storage> DataDeque
            for crate::Destaque<T, CAP, $IDX, S> {
            fn queue_pop_back(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop_back()
            }
            fn queue_push_front(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push_front(element)
            }
        }
        // unsafe alternative without T: Clone
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, const CAP: usize, S: Storage> DataQueue for crate::Destaque<T, CAP, $IDX, S> {
            fn queue_pop(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop_front()
            }
            fn queue_push(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push_back(element)
            }
        }
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, const CAP: usize, S: Storage> DataDeque for crate::Destaque<T, CAP, $IDX, S> {
            fn queue_pop_back(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop_back()
            }
            fn queue_push_front(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push_front(element)
            }
        }

        /* stack, desta */

        // safe alternative with T: Clone
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, const CAP: usize, S: Storage> DataStack for Destaque<T, CAP, $IDX, S> {
            fn stack_pop(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop_back()
            }
            fn stack_push(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push_back(element)
            }
        }
        #[cfg(any(feature = "safe_data", not(feature = "unsafe_ptr")))]
        impl<T: Clone, const CAP: usize, S: Storage> DataDesta for Destaque<T, CAP, $IDX, S> {
            fn stack_pop_front(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop_front()
            }
            fn stack_push_front(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push_front(element)
            }
        }
        // unsafe alternative without T: Clone
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, const CAP: usize, S: Storage> DataStack for Destaque<T, CAP, $IDX, S> {
            fn stack_pop(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop_back()
            }
            fn stack_push(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push_back(element)
            }
        }
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_ptr"))]
        impl<T, const CAP: usize, S: Storage> DataDesta for Destaque<T, CAP, $IDX, S> {
            fn stack_pop_front(&mut self)
                -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
                self.pop_front()
            }
            fn stack_push_front(&mut self, element: <Self as DataCollection>::Element)
                -> Result<(), NotEnoughSpace> {
                self.push_front(element)
            }
        }
        /* impl From<IntoIterator<Item = T>> */

        impl<T: Default, I, const CAP: usize> From<I> for Destaque<T, CAP, $IDX, Bare>
        where
            I: IntoIterator<Item = T>,
        {
            /// Returns a queue filled with an iterator, in the stack.
            /// # Examples
            /// ```
            #[doc = "# use devela::Destaque" $IDX:camel ";"]
            #[doc = "let q: Destaque" $IDX:camel "<_, 3> = [1, 2, 3].into();"]
            /// ```
            fn from(iterator: I) -> Destaque<T, CAP, $IDX, Bare> {
                let mut q = Destaque::<T, CAP, $IDX, Bare>::default();
                let _ = q.extend_back(iterator);
                q
            }
        }

        #[cfg(feature = "alloc")]
        impl<T: Default, I, const CAP: usize> From<I> for Destaque<T, CAP, $IDX, Boxed>
        where
            I: IntoIterator<Item = T>,
        {
            /// Returns a queue filled with an iterator, in the heap.
            /// # Examples
            /// ```
            #[doc = "# use devela::{Boxed, Destaque" $IDX:camel "};"]
            #[doc = "let q: Destaque" $IDX:camel "<_, 3, Boxed> = [1, 2, 3].into();"]
            /// ```
            fn from(iterator: I) -> Destaque<T, CAP, $IDX, Boxed> {
                let mut q = Destaque::<T, CAP, $IDX, Boxed>::default();
                let _ = q.extend_back(iterator);
                q
            }
        }

        /* iterators */

        impl<'s, T, const CAP: usize, S: Storage> Iterator for DestaqueIter<'s, T, CAP, $IDX, S> {
            type Item = &'s T;
            /// Iterates over shared references.
            /// # Example
            /// ```
            #[doc = "# use devela::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, 4>::from([1, 2]);"]
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
                    Some(&self.destaque.data[self.destaque.idx_front(self.idx as $IDX)])
                };
                self.idx += 1;
                item
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.destaque.len() as usize, Some(self.destaque.len() as usize))
            }
        }
        impl<'s, T, const CAP: usize, S: Storage> ExactSizeIterator for DestaqueIter<'s, T, CAP, $IDX, S> {}

        impl<'s, T, const CAP: usize, S: Storage> DoubleEndedIterator for DestaqueIter<'s, T, CAP, $IDX, S> {
            /// Iterates over shared references.
            /// # Example
            /// ```
            #[doc = "# use devela::Destaque" $IDX:camel ";"]
            #[doc = "let mut q = Destaque" $IDX:camel "::<i32, 4>::from([1, 2]);"]
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
                    Some(&self.destaque.data[self.destaque.idx_back(self.idx as $IDX)])
                };
                self.idx += 1;
                item
            }
        }

        /* PartialOrd, Ord */

        // T: PartialOrd
        impl<T: PartialOrd, const CAP: usize, S: Storage> PartialOrd for Destaque<T, CAP, $IDX, S>
        where
            S::Stored<[T; CAP]>: PartialOrd,
        {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.iter().partial_cmp(other.iter())
            }
        }

        // T: Ord
        impl<T: Ord, const CAP: usize, S: Storage> Ord for Destaque<T, CAP, $IDX, S>
        where
            S::Stored<[T; CAP]>: Ord,
        {
            fn cmp(&self, other: &Self) -> Ordering {
                self.iter().cmp(other.iter())
            }
        }
    }};
}
impl_destaque!();

// T: Clone
impl<T: Clone, const CAP: usize, IDX: Clone, S: Storage> Clone for Destaque<T, CAP, IDX, S>
where
    S::Stored<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            front: self.front.clone(),
            back: self.back.clone(),
            len: self.len.clone(),
        }
    }
}

// T: Copy
impl<T: Copy, const CAP: usize, IDX: Copy, S: Storage> Copy for Destaque<T, CAP, IDX, S> where
    S::Stored<[T; CAP]>: Copy
{
}

// T: Debug
impl<T: fmt::Debug, const CAP: usize, IDX: fmt::Debug, S: Storage> fmt::Debug
    for Destaque<T, CAP, IDX, S>
where
    S::Stored<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Destaque")
            .field("CAP", &CAP)
            .field("len", &self.len)
            .field("front", &self.front)
            .field("back", &self.back)
            .field("data", &self.data)
            .finish()
    }
}

// T: PartialEq
impl<T: PartialEq, const CAP: usize, IDX: PartialEq, S: Storage> PartialEq
    for Destaque<T, CAP, IDX, S>
where
    S::Stored<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.len == other.len
            && self.front == other.front
            && self.back == other.back
    }
}
// T: Eq
impl<T: Eq, const CAP: usize, IDX: Eq, S: Storage> Eq for Destaque<T, CAP, IDX, S> where
    S::Stored<[T; CAP]>: Eq
{
}

// T: Default, S: Bare
impl<T: Default, const CAP: usize, IDX: Default> Default for Destaque<T, CAP, IDX, Bare> {
    /// Returns an empty queue, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self {
            data: Array::default(),
            front: IDX::default(),
            back: IDX::default(),
            len: IDX::default(),
        }
    }
}

// T: ConstInit, S: Bare
impl<T: ConstInit, const CAP: usize, IDX: ConstInit> ConstInit for Destaque<T, CAP, IDX, Bare> {
    /// Returns an empty stack, allocated in the stack,
    /// using the default value to fill the remaining free data.
    const INIT: Self = Self {
        data: Array::INIT,
        front: IDX::INIT,
        back: IDX::INIT,
        len: IDX::INIT,
    };
}

// T: Default, S: Boxed
#[cfg(feature = "alloc")]
impl<T: Default, const CAP: usize, IDX: Default> Default for Destaque<T, CAP, IDX, Boxed> {
    /// Returns an empty queue, allocated in the heap,
    /// using the default value to fill the remaining free data.
    /// # Examples
    /// ```
    /// # use devela::{Boxed, DestaqueU8};
    /// let mut q = DestaqueU8::<i32, 100, Boxed>::default();
    /// ```
    fn default() -> Self {
        Self {
            data: Array::default(),
            front: IDX::default(),
            back: IDX::default(),
            len: IDX::default(),
        }
    }
}
