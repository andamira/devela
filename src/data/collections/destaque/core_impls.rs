// devela::data::collections::destaque::methods
//
//!
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    data::{Array, Destaque},
    mem::Storage,
};
use core::fmt;

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize> Clone for Destaque<T, S, CAP>
where
    S::Stored<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
            front: self.front,
            back: self.back,
            len: self.len,
        }
    }
}

// T:Copy
impl<T: Copy, S: Storage, const CAP: usize> Copy for Destaque<T, S, CAP> where
    S::Stored<[T; CAP]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize> fmt::Debug for Destaque<T, S, CAP>
where
    S::Stored<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![DirectDestaque]);
        debug
            .field("CAP", &CAP)
            .field("len", &self.len)
            .field("front", &self.front)
            .field("back", &self.back);

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
impl<T: PartialEq, S: Storage, const CAP: usize> PartialEq for Destaque<T, S, CAP>
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
impl<T: Eq, S: Storage, const CAP: usize> Eq for Destaque<T, S, CAP> where S::Stored<[T; CAP]>: Eq {}

// T:PartialOrd
use core::cmp::Ordering;
impl<T: PartialOrd, S: Storage, const CAP: usize> PartialOrd for Destaque<T, S, CAP>
where
    S::Stored<[T; CAP]>: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}
// T:Ord
impl<T: Ord, S: Storage, const CAP: usize> Ord for Destaque<T, S, CAP>
where
    S::Stored<[T; CAP]>: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

// S:() + T:Default
impl<T: Default, const CAP: usize> Default for Destaque<T, (), CAP> {
    /// Returns an empty queue, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self {
            array: Array::default(),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "alloc")]
impl<T: Default, const CAP: usize> Default for Destaque<T, Boxed, CAP> {
    /// Returns an empty queue, allocated in the heap,
    /// using the default value to fill the remaining free data.
    /// # Examples
    /// ```
    /// # use devela::data::BoxedDestaque;
    /// let mut s = BoxedDestaque::<i32, 100>::default();
    /// ```
    fn default() -> Self {
        Self {
            array: Array::default(),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

impl<T: Default, I, const CAP: usize> From<I> for Destaque<T, (), CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a queue filled with an iterator, in the stack.
    /// # Examples
    /// ```
    /// # use devela::data::DirectDestaque;
    /// let s: DirectDestaque<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> Destaque<T, (), CAP> {
        let mut s = Destaque::<T, (), CAP>::default();
        let _ = s.extend_back(iterator);
        s
    }
}

#[cfg(feature = "alloc")]
impl<T: Default, I, const CAP: usize> From<I> for Destaque<T, Boxed, CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a queue filled with an iterator, in the heap.
    /// # Examples
    /// ```
    /// # use devela::data::BoxedDestaque;
    /// let s: BoxedDestaque<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> Destaque<T, Boxed, CAP> {
        let mut s = Destaque::<T, Boxed, CAP>::default();
        let _ = s.extend_back(iterator);
        s
    }
}
