// devela::data::collections::array::methods
//
//! Arrays.
//

use crate::{
    data::{array_init, Array},
    mem::{Direct, Storage},
};

#[allow(unused)]
#[cfg(feature = "alloc")]
use crate::{
    _deps::alloc::{boxed::Box, vec::Vec},
    mem::Boxed,
};

// ``
impl<T, S: Storage, const LEN: usize> Array<T, S, LEN> {
    /// Returns a new `Array` from the given primitive `array`.
    pub fn new(array: [T; LEN]) -> Self {
        Self {
            array: array.into(),
        }
    }
}

// S:()
impl<T, const LEN: usize> Array<T, (), LEN> {
    /// Returns a new [`DirectArray`] from the given primitive `array` in compile-time.
    pub const fn new_const(array: [T; LEN]) -> Self {
        Self {
            array: Direct::new(array),
        }
    }
}

// `S:() + T:Clone`
impl<T: Clone, const LEN: usize> Array<T, (), LEN> {
    /// Returns an array, allocated in the stack, filled with `element`, cloned.
    /// # Examples
    /// ```
    /// # use devela::data::Array;
    /// let a = Array::<_, (), 16>::with_cloned(0);
    /// ```
    pub fn with_cloned(element: T) -> Self {
        let array = Direct::new(array_init!(clone [T; LEN], "safe_data", "unsafe_array", element));
        Self { array }
    }
}

// `S:() + T:Copy`
impl<T: Copy, const LEN: usize> Array<T, (), LEN> {
    /// Returns an array, allocated in the stack, filled with `element`, copied, in compile-time.
    /// # Examples
    /// ```
    /// # use devela::data::Array;
    /// const A: Array<i32, (), 16> = Array::with_copied(0);
    /// ```
    pub const fn with_copied(element: T) -> Self {
        let array = Direct::new([element; LEN]);
        Self { array }
    }
}

// `S:Boxed + T:Clone`
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Clone, const LEN: usize> Array<T, Boxed, LEN> {
    /// Returns an array, allocated in the heap, filled with `element`, cloned.
    /// # Examples
    /// ```
    /// # use devela::data::BoxedArray;
    /// let mut a = BoxedArray::<_, 1_000>::with_cloned(0);
    /// ```
    pub fn with_cloned(element: T) -> Self {
        let array = array_init!(clone_heap [T; LEN], "safe_data", "unsafe_array", element);
        Self { array }
    }
}

// `T: PartialEq`
impl<T: PartialEq, S: Storage, const CAP: usize> Array<T, S, CAP> {
    /// Returns true if the array contains `element`.
    /// # Examples
    /// ```
    /// # use devela::all::Array;
    /// let a = Array::<_, (), 5>::new([5, 78, 42, 33, 9]);
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const LEN: usize> Array<T, Boxed, LEN> {
    /// Returns the inner boxed primitive array.
    pub fn into_array(self) -> Box<[T; LEN]> {
        self.array
    }
}
// `S: ()`
impl<T, const LEN: usize> Array<T, (), LEN> {
    /// Returns the inner [`Direct`] primitive array.
    pub fn into_array(self) -> [T; LEN] {
        self.array.into_inner()
    }
}
// `S: (), T:Copy`
impl<T: Copy, const LEN: usize> Array<T, (), LEN> {
    /// Returns the inner [`Direct`] primitive array in compile-time.
    pub const fn into_array_const(self) -> [T; LEN] {
        self.array.into_inner_const()
    }
}
