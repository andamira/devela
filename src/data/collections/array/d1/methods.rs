// devela::data::collections::array::d1::methods
//
//! 1-dimensional array methods
//

#[allow(unused_imports)]
#[cfg(feature = "alloc")]
use crate::{
    data::Vec,
    mem::{Box, Boxed},
};
use crate::{
    data::{array_from_fn, array_init, Array, DataError::ElementNotFound, DataResult as Result},
    mem::{Bare, BareBox, Storage},
};

/* constructors */

// S
impl<T, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Returns a new `Array` from the given primitive `array`.
    #[inline]
    pub fn new(array: [T; CAP]) -> Self {
        Self { data: array.into() }
    }

    /// Returns a new `Array`, where each element `T` is the returned value from `f`.
    ///
    /// See: `array::`[`from_fn`][core::array::from_fn]
    #[inline]
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        Self { data: array_from_fn(f).into() }
    }
    // WAIT [array_try_from_fn](https://github.com/rust-lang/rust/issues/89379)
}

// T, S: Bare
impl<T, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns a new [`Array`] allocated in the stack,
    /// from the given primitive `array` in compile-time.
    #[inline]
    pub const fn new_bare(array: [T; CAP]) -> Self {
        Self { data: BareBox::new(array) }
    }
}

// T: Clone, S: Bare
impl<T: Clone, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns an array, allocated in the stack, filled with `element`, cloned.
    /// # Examples
    /// ```
    /// # use devela::data::Array;
    /// let a = Array::<_, 16>::with_cloned(0);
    /// ```
    #[inline]
    pub fn with_cloned(element: T) -> Self {
        let data = BareBox::new(array_init!(clone [T; CAP], "safe_data", "unsafe_array", element));
        Self { data }
    }
}

// T: Copy, S: Bare
impl<T: Copy, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns an array, allocated in the stack, filled with `element`, copied, in compile-time.
    /// # Examples
    /// ```
    /// # use devela::data::Array;
    /// const A: Array<i32, 16> = Array::with_copied(0);
    /// ```
    #[inline]
    pub const fn with_copied(element: T) -> Self {
        let data = BareBox::new([element; CAP]);
        Self { data }
    }
}

// T, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const CAP: usize> Array<T, CAP, Boxed> {
    /// Returns a new `Array` from the given `boxed_array`.
    #[inline]
    pub fn new_boxed(boxed_array: Box<[T; CAP]>) -> Self {
        Array { data: boxed_array }
    }
}

// T:Clone, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Clone, const CAP: usize> Array<T, CAP, Boxed> {
    /// Returns an array, allocated in the heap, filled with `element`, cloned.
    /// # Examples
    /// ```
    /// # use devela::{Array, Boxed};
    /// let mut a = Array::<_, 1_000, Boxed>::with_cloned(0);
    /// ```
    #[inline]
    pub fn with_cloned(element: T) -> Self {
        let data = array_init!(clone_heap [T; CAP], "safe_data", "unsafe_array", element);
        Self { data }
    }
}

/* methods */

// T: Clone, S
impl<T: Clone, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Fills all elements of the array with the given `element`.
    #[inline]
    pub fn fill(&mut self, element: T) {
        self.iter_mut().for_each(|i| *i = element.clone());
    }
}

// T: Default, S
impl<T: Default, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Fills all elements of the array with the default value.
    #[inline]
    pub fn fill_default(&mut self) {
        self.iter_mut().for_each(|i| *i = T::default());
    }
}

// T: PartialEq, S
impl<T: PartialEq, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Returns true if the array contains `element`.
    /// # Examples
    /// ```
    /// # use devela::Array;
    /// let a = Array::<_, 5>::new([5, 78, 42, 33, 9]);
    /// assert![a.contains(&9)];
    /// assert![!a.contains(&8)];
    /// ```
    #[inline]
    #[must_use]
    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|n| n == element)
    }

    /// Finds the index of the first occurrence of `element` in the array.
    /// # Examples
    /// ```
    /// # use devela::Array;
    /// let a = Array::<_, 5>::new([5, 78, 42, 33, 9]);
    /// assert_eq![a.find_index(&9), Some(4)];
    /// assert_eq![a.find_index(&8), None];
    /// ```
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the `element` can't be found.
    #[inline]
    pub fn find_index(&self, element: &T) -> Result<usize> {
        self.iter()
            .enumerate()
            .find_map(|(i, n)| (n == element).then_some(i))
            .ok_or(ElementNotFound)
    }
}

// T, S
impl<T, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Returns the capacity of the array.
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// Returns a shared slice containing the entire array.
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

// T, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const CAP: usize> Array<T, CAP, Boxed> {
    /// Returns the inner [`Box`]ed primitive array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> Box<[T; CAP]> {
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
// T, S: Bare
impl<T, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns the inner [`BareBox`]ed primitive array.
    #[inline]
    #[must_use]
    pub fn into_array(self) -> [T; CAP] {
        self.data.into_inner()
    }

    /// Returns a slice containing the entire array in compile time.
    ///
    /// It allows to sidestep `Deref` coercion for indexing purposes.
    #[inline]
    #[must_use]
    pub const fn as_bare_slice(&self) -> &[T] {
        self.data.as_ref() // const method on BareBox
    }
}
// T:Copy, S: Bare
impl<T: Copy, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns the inner [`BareBox`]ed primitive array in compile-time.
    #[inline]
    #[must_use]
    pub const fn into_array_copy(self) -> [T; CAP] {
        self.data.into_inner_copy()
    }
}
