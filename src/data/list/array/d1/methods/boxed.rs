// devela::data::list::array::d1::methods::boxed
//
//! 1-dimensional array *Boxed* methods.
//
// TOC
// - constructors
// - methods

use crate::{Array, Box, Boxed, Vec, array_init};

/// # *Boxed* constructors
impl<T, const CAP: usize> Array<T, CAP, Boxed> {
    /// Returns a new `Array` from the given `boxed_array`.
    pub fn new_boxed(boxed_array: Box<[T; CAP]>) -> Self {
        Array { data: boxed_array }
    }

    /// Returns an array, allocated in the heap, filled with `element`, cloned.
    ///
    /// # Example
    /// ```
    /// # use devela::{Array, Boxed};
    /// let mut a = Array::<_, 1_000, Boxed>::with_cloned(0);
    /// ```
    pub fn with_cloned(element: T) -> Self
    where
        T: Clone,
    {
        let data = array_init!(clone_heap [T; CAP], "safe_data", "unsafe_array", element);
        Self { data }
    }
}

/// # *Boxed* methods
impl<T, const CAP: usize> Array<T, CAP, Boxed> {
    /// Returns the inner [`Box`]ed primitive array.
    #[must_use]
    pub fn into_array(self) -> Box<[T; CAP]> {
        self.data
    }

    /// Returns the inner [`Box`]ed primitive array as a slice.
    #[must_use]
    pub fn into_slice(self) -> Box<[T]> {
        self.data
    }

    /// Returns the inner [`Box`]ed primitive array as a `Vec`.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.into_slice().into_vec()
    }
}
