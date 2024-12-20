// devela::data::collections::array::d1::methods
//
//! 1-dimensional array methods
//
// TOC
// - constructors
// - methods
// - methods for Option<T>

use crate::{array_from_fn, array_init, iif, Array, Bare, BareBox, Storage};
#[allow(unused_imports)]
#[cfg(feature = "alloc")]
use crate::{Box, Boxed, Vec};
#[cfg(feature = "data")]
use crate::{
    DataError::{ElementNotFound, MismatchedIndices, OutOfBounds},
    DataResult as Result,
};

/* constructors */
// -----------------------------------------------------------------------------

// S
impl<T, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Returns a new `Array` from the given primitive `array`.
    pub fn new(array: [T; CAP]) -> Self {
        Self { data: array.into() }
    }

    /// Returns a new `Array`, where each element `T` is the returned value from `f`.
    ///
    /// See: `array::`[`from_fn`][core::array::from_fn]
    pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {
        Self { data: array_from_fn(f).into() }
    }
    // WAIT [array_try_from_fn](https://github.com/rust-lang/rust/issues/89379)
}

// T, S: Bare
impl<T, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns a new [`Array`] allocated in the stack,
    /// from the given primitive `array` in compile-time.
    pub const fn new_bare(array: [T; CAP]) -> Self {
        Self { data: BareBox::new(array) }
    }
}

// T: Clone, S: Bare
impl<T: Clone, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns an array, allocated in the stack, filled with `element`, cloned.
    ///
    /// # Example
    /// ```
    /// # use devela::data::Array;
    /// let a = Array::<_, 16>::with_cloned(0);
    /// ```
    pub fn with_cloned(element: T) -> Self {
        let data = BareBox::new(array_init!(clone [T; CAP], "safe_data", "unsafe_array", element));
        Self { data }
    }
}

// T: Copy, S: Bare
impl<T: Copy, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns an array, allocated in the stack, filled with `element`, copied, in compile-time.
    ///
    /// # Example
    /// ```
    /// # use devela::data::Array;
    /// const A: Array<i32, 16> = Array::with_copied(0);
    /// ```
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
    pub fn new_boxed(boxed_array: Box<[T; CAP]>) -> Self {
        Array { data: boxed_array }
    }
}

// T:Clone, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Clone, const CAP: usize> Array<T, CAP, Boxed> {
    /// Returns an array, allocated in the heap, filled with `element`, cloned.
    ///
    /// # Example
    /// ```
    /// # use devela::{Array, Boxed};
    /// let mut a = Array::<_, 1_000, Boxed>::with_cloned(0);
    /// ```
    pub fn with_cloned(element: T) -> Self {
        let data = array_init!(clone_heap [T; CAP], "safe_data", "unsafe_array", element);
        Self { data }
    }
}

/* methods */
// -----------------------------------------------------------------------------

// T: Clone, S
impl<T: Clone, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Fills all elements of the array with the given `element`.
    pub fn fill(&mut self, element: T) {
        self.iter_mut().for_each(|i| *i = element.clone());
    }
}

// T: Default, S
impl<T: Default, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Fills all elements of the array with the default value.
    pub fn fill_default(&mut self) {
        self.iter_mut().for_each(|i| *i = T::default());
    }
}

// T: PartialEq, S
impl<T: PartialEq, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Returns `true` if the array contains `element`.
    ///
    /// # Example
    /// ```
    /// # use devela::Array;
    /// let a = Array::<_, 5>::new([5, 78, 42, 33, 9]);
    /// assert![a.contains(&9)];
    /// assert![!a.contains(&8)];
    /// ```
    #[must_use]
    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|n| n == element)
    }

    /// Returns `true` if the array contains `element`,
    /// from the `start` index (inclusive).
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= CAP`.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn contains_from(&self, element: &T, start: usize) -> Result<bool> {
        iif![start >= CAP; return Err(OutOfBounds(Some(start)))];
        Ok(self.iter().skip(start).any(|n| n == element))
    }

    /// Returns `true` if the array contains `element`,
    /// from the `start` index (inclusive).
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `end >= CAP`.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn contains_until(&self, element: &T, end: usize) -> Result<bool> {
        iif![end >= CAP; return Err(OutOfBounds(Some(end)))];
        Ok(self.iter().take(end + 1).any(|n| n == element))
    }

    /// Returns `true` if the array contains `element`,
    /// between the range `start..=end` (inclusive).
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if either `start` or `end` `>= CAP`,
    /// or [`MismatchedIndices`] if `start > end`.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn contains_between(&self, element: &T, start: usize, end: usize) -> Result<bool> {
        iif![start >= CAP; return Err(OutOfBounds(Some(start)))];
        iif![end >= CAP; return Err(OutOfBounds(Some(end)))];
        iif![start > end; return Err(MismatchedIndices)];
        Ok(self.iter().skip(start).take(end - start + 1).any(|n| n == element))
    }

    /// Finds the index of the first occurrence of `element` in the array.
    ///
    /// # Example
    /// ```
    /// # use devela::{Array, DataError::ElementNotFound};
    /// let a = Array::<_, 5>::new([5, 78, 42, 33, 9]);
    /// assert_eq![a.find_index(&9), Ok(4)];
    /// assert_eq![a.find_index(&8), Err(ElementNotFound)];
    /// ```
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the `element` can't be found.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
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
    #[must_use]
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// Returns a shared slice containing the entire array.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Returns an exclusive slice containing the entire array.
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
// T, S: Bare
impl<T, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns the inner [`BareBox`]ed primitive array.
    #[must_use]
    pub fn into_array(self) -> [T; CAP] {
        self.data.into_inner()
    }

    /// Returns a slice containing the entire array in compile time.
    ///
    /// It allows to sidestep `Deref` coercion for indexing purposes.
    #[must_use]
    pub const fn as_bare_slice(&self) -> &[T] {
        self.data.as_ref() // const method on BareBox
    }
}
// T:Copy, S: Bare
impl<T: Copy, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns the inner [`BareBox`]ed primitive array in compile-time.
    #[must_use]
    pub const fn into_array_copy(self) -> [T; CAP] {
        self.data.into_inner_copy()
    }
}

/* methods for Option<T> */
// -----------------------------------------------------------------------------

// Option<T>, S
/// # Operations depending on `Option<T>`.
impl<T, const CAP: usize, S: Storage> Array<Option<T>, CAP, S> {
    /// Takes out some element at `index`, leaving `None` in its place.
    #[must_use]
    pub fn take(&mut self, index: usize) -> Option<T> {
        self.get_mut(index)?.take()
    }

    /// Replaces some element at `index` with `value`, returning the old one.
    #[must_use]
    pub fn replace(&mut self, index: usize, value: T) -> Option<T> {
        self.get_mut(index)?.replace(value)
    }

    /// Sets the element at `index` to `None`.
    pub fn unset(&mut self, index: usize) {
        self[index] = None;
    }

    /// Clears the array by setting all elements to `None`.
    pub fn clear(&mut self) {
        self.iter_mut().for_each(|i| *i = None);
    }

    /// Returns the number of `Some` elements in the array.
    #[must_use]
    pub fn count_some(&self) -> usize {
        self.iter().filter(|opt| opt.is_some()).count()
    }
    /// Returns the number of `None` elements in the array.
    #[must_use]
    pub fn count_none(&self) -> usize {
        self.iter().filter(|opt| opt.is_none()).count()
    }

    /// Returns the number of `None` elements in the array.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.iter().all(|opt| opt.is_some())
    }

    /// Returns the number of `None` elements in the array.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.iter().all(|opt| opt.is_some())
    }

    /// Returns the index of the first `None` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn first_none(&self) -> Result<usize> {
        self.iter().position(|opt| opt.is_none()).ok_or(ElementNotFound)
    }
    /// Returns some reference to the first `None` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn first_none_ref(&self) -> Result<&Option<T>> {
        self.iter().find(|opt| opt.is_none()).ok_or(ElementNotFound)
    }
    /// Returns some exclusive reference to the first `None` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn first_none_mut(&mut self) -> Result<&mut Option<T>> {
        self.iter_mut().find(|opt| opt.is_none()).ok_or(ElementNotFound)
    }

    /// Returns the index of the first `Some` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn first_some(&self) -> Result<usize> {
        self.iter().position(|opt| opt.is_some()).ok_or(ElementNotFound)
    }
    /// Returns some reference to the first `Some` element
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn first_some_ref(&self) -> Result<&Option<T>> {
        self.iter().find(|opt| opt.is_some()).ok_or(ElementNotFound)
    }
    /// Returns some exclusive reference to the first `Some` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    #[cfg(feature = "data")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
    pub fn first_some_mut(&mut self) -> Result<&mut Option<T>> {
        self.iter_mut().find(|opt| opt.is_some()).ok_or(ElementNotFound)
    }
}

// Option<T: Clone>, S
/// # Methods depending on `Option<T: Clone>`.
impl<T: Clone, const CAP: usize> Array<Option<T>, CAP, Bare> {
    /// Fills all `None` elements of the array with the given cloned `value`.
    pub fn fill_none(&mut self, value: T) {
        self.iter_mut().filter(|opt| opt.is_none()).for_each(|opt| *opt = Some(value.clone()));
    }
}

// Option<T>, S: Bare
/// # Methods depending on `Option<T: Copy>`.
impl<T, const CAP: usize> Array<Option<T>, CAP, Bare> {
    /// Checks if all elements are `None`, returning early if a `Some` is found.
    pub const fn is_bare_empty(&self) -> bool {
        let mut n = 0;
        while n <= CAP {
            iif![self.as_bare_slice()[n].is_some(); return false];
            n += 1;
        }
        true
    }

    /// Checks if all elements are `Some`, returning early if a `None` is found.
    pub const fn is_bare_full(&self) -> bool {
        let mut n = 0;
        while n <= CAP {
            iif![self.as_bare_slice()[n].is_none(); return false];
            n += 1;
        }
        true
    }
}
