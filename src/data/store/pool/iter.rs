// devela/src/data/store/pool/iter.rs
//
//! Defines [`PoolIter`].
//

use crate::{IteratorExactSize, IteratorFused, Mem, is};

#[doc = crate::_tags!(iterator data_structure)]
/// An iterator over occupied slots in a pool.
#[doc = crate::_doc_meta!{
    location("data/store"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: PoolIter<&[Option<char>]> = 12|96),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: PoolIter<&[Option<char>]> = 24|192),
}]
/// Returned by [`pool!`][crate::pool]'s methods [`iter`] and [`iter_mut`],
/// and by reference iteration.
///
/// [`iter`]: crate::PoolExample::iter
/// [`iter_mut`]: crate::PoolExample::iter_mut
#[must_use]
#[derive(Clone, Debug)]
pub struct PoolIter<S> {
    slots: S,
    remaining: usize,
}
impl<S> PoolIter<S> {
    #[doc(hidden)]
    pub const fn _new(slots: S, remaining: usize) -> Self {
        Self { slots, remaining }
    }
    /// Returns the exact number of occupied values not yet yielded.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.remaining
    }
    /// Returns whether no occupied values remain.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.remaining == 0
    }
    const fn _size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

/* ref */

impl<'a, T> PoolIter<&'a [Option<T>]> {
    /// Returns the next occupied value.
    #[must_use]
    pub const fn next(&mut self) -> Option<&'a T> {
        is! { self.remaining == 0, return None }
        loop {
            let slots = self.slots;
            let Some((slot, rest)) = slots.split_first() else {
                return None;
            };
            self.slots = rest;
            if let Some(value) = slot.as_ref() {
                self.remaining -= 1;
                return Some(value);
            }
        }
    }
}
impl<'a, T> Iterator for PoolIter<&'a [Option<T>]> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        PoolIter::<&'a [Option<T>]>::next(self)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self._size_hint()
    }
}
impl<T> IteratorExactSize for PoolIter<&[Option<T>]> {
    fn len(&self) -> usize {
        PoolIter::len(self)
    }
}
impl<T> IteratorFused for PoolIter<&[Option<T>]> {}

/* mut */

impl<'a, T> PoolIter<&'a mut [Option<T>]> {
    /// Returns the next occupied value exclusively.
    #[must_use]
    pub const fn next(&mut self) -> Option<&'a mut T> {
        is! { self.remaining == 0, return None }
        loop {
            let slots = Mem::replace(&mut self.slots, &mut []);
            let Some((slot, rest)) = slots.split_first_mut() else {
                return None;
            };
            self.slots = rest;
            if let Some(value) = slot.as_mut() {
                self.remaining -= 1;
                return Some(value);
            }
        }
    }
}
impl<'a, T> Iterator for PoolIter<&'a mut [Option<T>]> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        PoolIter::<&'a mut [Option<T>]>::next(self)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self._size_hint()
    }
}
impl<T> IteratorExactSize for PoolIter<&mut [Option<T>]> {
    fn len(&self) -> usize {
        PoolIter::len(self)
    }
}
impl<T> IteratorFused for PoolIter<&mut [Option<T>]> {}
