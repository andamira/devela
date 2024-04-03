// devela::data::collections::array::d1::methods
//
//! 1-dimensional array methods
//

#[allow(unused_imports)]
#[cfg(feature = "alloc")]
use crate::{
    _liballoc::{boxed::Box, vec::Vec},
    mem::Boxed,
};
use crate::{
    data::{array_init, Array},
    mem::{Bare, BareBox, Storage},
};

impl<T, const LEN: usize, S: Storage> Array<T, LEN, S> {
    /// Returns a new `Array` from the given primitive `array`.
    #[inline]
    pub fn new(array: [T; LEN]) -> Self {
        Self { data: array.into() }
    }
}

// S:Bare
impl<T, const LEN: usize> Array<T, LEN, Bare> {
    /// Returns a new [`Array`] allocated in the stack,
    /// from the given primitive `array` in compile-time.
    #[inline]
    pub const fn new_bare(array: [T; LEN]) -> Self {
        Self {
            data: BareBox::new(array),
        }
    }
}

// S:Bare + T:Clone
impl<T: Clone, const LEN: usize> Array<T, LEN, Bare> {
    /// Returns an array, allocated in the stack, filled with `element`, cloned.
    /// # Examples
    /// ```
    /// # use devela::data::Array;
    /// let a = Array::<_, 16>::with_cloned(0);
    /// ```
    #[inline]
    pub fn with_cloned(element: T) -> Self {
        let data = BareBox::new(array_init!(clone [T; LEN], "safe_data", "unsafe_array", element));
        Self { data }
    }
}

// S:Bare + T:Copy
impl<T: Copy, const LEN: usize> Array<T, LEN, Bare> {
    /// Returns an array, allocated in the stack, filled with `element`, copied, in compile-time.
    /// # Examples
    /// ```
    /// # use devela::data::Array;
    /// const A: Array<i32, 16> = Array::with_copied(0);
    /// ```
    #[inline]
    pub const fn with_copied(element: T) -> Self {
        let data = BareBox::new([element; LEN]);
        Self { data }
    }
}

// S:Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const LEN: usize> Array<T, LEN, Boxed> {
    /// Returns a new `Array` from the given `boxed_array`.
    #[inline]
    pub fn new_boxed(boxed_array: Box<[T; LEN]>) -> Self {
        Array { data: boxed_array }
    }
}

// S:Boxed + T:Clone
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Clone, const LEN: usize> Array<T, LEN, Boxed> {
    /// Returns an array, allocated in the heap, filled with `element`, cloned.
    /// # Examples
    /// ```
    /// # use devela::all::{Array, Boxed};
    /// let mut a = Array::<_, 1_000, Boxed>::with_cloned(0);
    /// ```
    #[inline]
    pub fn with_cloned(element: T) -> Self {
        let data = array_init!(clone_heap [T; LEN], "safe_data", "unsafe_array", element);
        Self { data }
    }
}

// T:Clone
impl<T: Clone, const LEN: usize, S: Storage> Array<T, LEN, S> {
    /// Fills all elements of the array with the given `element`.
    #[inline]
    pub fn fill(&mut self, element: T) {
        self.iter_mut().for_each(|i| *i = element.clone());
    }
}

// T:Default
impl<T: Default, const LEN: usize, S: Storage> Array<T, LEN, S> {
    /// Fills all elements of the array with the default value.
    #[inline]
    pub fn fill_default(&mut self) {
        self.iter_mut().for_each(|i| *i = T::default());
    }
}

// T:PartialEq
impl<T: PartialEq, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Returns true if the array contains `element`.
    /// # Examples
    /// ```
    /// # use devela::all::Array;
    /// let a = Array::<_, 5>::new([5, 78, 42, 33, 9]);
    /// assert![a.contains(&9)];
    /// assert![!a.contains(&8)];
    /// ```
    #[inline]
    #[must_use]
    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|n| n == element)
    }
}

impl<T, const LEN: usize, S: Storage> Array<T, LEN, S> {
    /// Returns the number of elements in the array.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        LEN
    }

    /// Returns `true` if the array has a length of 0.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        LEN == 0
    }

    /// Returns a slice containing the entire array.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Returns an exclusive slice containing the entire array.
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }
}

// S:Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const LEN: usize> Array<T, LEN, Boxed> {
    /// Returns the inner [`Box`]ed primitive array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> Box<[T; LEN]> {
        self.data
    }
    /// Returns the inner [`Box`]ed primitive array as a slice.
    #[inline]
    #[must_use]
    pub fn into_slice(self) -> Box<[T]> {
        self.data
    }
    /// Returns the inner [`Box`]ed primitive array as a `Vec`.
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.into_slice().into_vec()
    }
}
// S:Bare
impl<T, const LEN: usize> Array<T, LEN, Bare> {
    /// Returns the inner [`BareBox`]ed primitive array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> [T; LEN] {
        self.data.into_inner()
    }
}
// S:Bare, T:Copy
impl<T: Copy, const LEN: usize> Array<T, LEN, Bare> {
    /// Returns the inner [`BareBox`]ed primitive array in compile-time.
    #[inline]
    #[must_use]
    pub const fn into_array_const(self) -> [T; LEN] {
        self.data.into_inner_const()
    }
}
