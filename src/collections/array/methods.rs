// devela::collections::array::methods
//
//! Arrays.
//

#[cfg(feature = "unsafe_init")]
use core::mem::{self, MaybeUninit};

use super::Array;
use crate::mem::{Direct, Storage};

#[allow(unused)]
#[cfg(feature = "alloc")]
use {
    crate::mem::Boxed,
    _alloc::{boxed::Box, vec::Vec},
};

// ``
impl<T, S: Storage, const LEN: usize> Array<T, S, LEN> {
    /// Returns an new `Array` from the given primitive `array`.
    pub fn new(array: [T; LEN]) -> Self {
        Self {
            array: array.into(),
        }
    }
}

// `S:() + T:Clone`
impl<T: Clone, const LEN: usize> Array<T, (), LEN> {
    /// Returns an array, allocated in the stack,
    /// filled with `element`, cloned.
    ///
    /// # Examples
    /// ```
    /// use devela::collections::Array;
    ///
    /// let s = Array::<_, (), 16>::with(0);
    /// ```
    pub fn with(element: T) -> Self {
        #[cfg(feature = "unsafe_init")]
        let data = Direct::new({
            let mut arr: [MaybeUninit<T>; LEN] = unsafe { MaybeUninit::uninit().assume_init() };

            for i in &mut arr[..] {
                let _ = i.write(element.clone());
            }

            // TEMP:FIX: can't use transmute for now:
            // - https://github.com/rust-lang/rust/issues/62875
            // - https://github.com/rust-lang/rust/issues/61956
            // mem::transmute::<_, [T; LEN]>(&arr)
            //
            // SAFETY: we've initialized all the elements
            unsafe { mem::transmute_copy::<_, [T; LEN]>(&arr) }
        });

        #[cfg(not(feature = "unsafe_init"))]
        let data = Direct::new(core::array::from_fn(|_| element.clone()));

        Self { array: data }
    }
}

// `S:Boxed + T:Clone`
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
impl<T: Clone, const LEN: usize> Array<T, Boxed, LEN> {
    /// Returns an empty stack, allocated in the heap,
    /// using `element` to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use devela::collections::BoxedArray;
    ///
    /// let mut s = BoxedArray::<_, 1_000>::with(0);
    /// ```
    pub fn with(element: T) -> Self {
        #[cfg(not(feature = "unsafe_init"))]
        let data = {
            let mut v = Vec::<T>::with_capacity(LEN);

            for _ in 0..LEN {
                v.push(element.clone());
            }

            let Ok(array) = v.into_boxed_slice().try_into() else {
                panic!("Can't turn the boxed slice into a boxed array");
            };
            array
        };

        #[cfg(feature = "unsafe_init")]
        let data = {
            let mut v = Vec::<T>::with_capacity(LEN);

            for _ in 0..LEN {
                v.push(element.clone());
            }

            let slice = v.into_boxed_slice();
            let raw_slice = Box::into_raw(slice);
            // SAFETY: pointer comes from using `into_raw`, and capacity is right.
            unsafe { Box::from_raw(raw_slice as *mut [T; LEN]) }
        };

        Self { array: data }
    }
}

// `T: PartialEq`
impl<T: PartialEq, S: Storage, const CAP: usize> Array<T, S, CAP> {
    /// Returns true if the array contains `element`.
    ///
    /// # Examples
    /// ```
    /// use devela::all::Array;
    ///
    /// let a = Array::<_, (), 5>::new([5, 78, 42, 33, 9]);
    ///
    /// assert![a.contains(&9)];
    /// assert![!a.contains(&8)];
    /// ```
    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|n| n == element)
    }
}

// ``
impl<T, S: Storage, const LEN: usize> Array<T, S, LEN> {
    /// Returns the number of elements in the array.
    #[inline]
    pub const fn len(&self) -> usize {
        LEN
    }

    /// Returns `true` if the array has a length of 0.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        LEN == 0
    }

    /// Returns a slice containing the entire array.
    pub fn as_slice(&self) -> &[T] {
        self.array.as_slice()
    }

    /// Returns an exclusive slice containing the entire array.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.array.as_mut_slice()
    }
}

// `S: Boxed`
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
impl<T, const LEN: usize> Array<T, Boxed, LEN> {
    /// Returns the inner boxed primitive array.
    pub fn into_array(self) -> Box<[T; LEN]> {
        self.array
    }
}
// `S: ()`
impl<T, const LEN: usize> Array<T, (), LEN> {
    /// Returns the inner boxed primitive array.
    pub fn into_array(self) -> [T; LEN] {
        self.array.0
    }
}
