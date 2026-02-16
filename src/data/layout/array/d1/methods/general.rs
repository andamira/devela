// devela::data::layout::array::d1::methods::general
//
//! 1-dimensional array general methods (Storage-independent).
//
// TOC
// - constructors
// - methods
// - T: PartialEq methods
// - Option<T> methods

use crate::{
    Array, ElementNotFound, IndexOutOfBounds, MismatchedBounds, Storage, array_from_fn, is,
};

/// # Constructors
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

/// # Methods
impl<T, const CAP: usize, S: Storage> Array<T, CAP, S> {
    /// Returns the total capacity of the array, equals `CAP`.
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

    /// Fills all elements of the array with the given `element`.
    pub fn fill(&mut self, element: T)
    where
        T: Clone,
    {
        self.iter_mut().for_each(|i| *i = element.clone());
    }

    /// Fills all elements of the array with the default value.
    pub fn fill_default(&mut self)
    where
        T: Default,
    {
        self.iter_mut().for_each(|i| *i = T::default());
    }
}

/// # `T: PartialEq` methods
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
    /// Returns [`IndexOutOfBounds`] if `start >= CAP`.
    pub fn contains_from(&self, element: &T, start: usize) -> Result<bool, IndexOutOfBounds> {
        is![start >= CAP; return Err(IndexOutOfBounds(Some(start)))];
        Ok(self.iter().skip(start).any(|n| n == element))
    }

    /// Returns `true` if the array contains `element`,
    /// from the `start` index (inclusive).
    ///
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if `end >= CAP`.
    pub fn contains_until(&self, element: &T, end: usize) -> Result<bool, IndexOutOfBounds> {
        is![end >= CAP; return Err(IndexOutOfBounds(Some(end)))];
        Ok(self.iter().take(end + 1).any(|n| n == element))
    }

    /// Returns `true` if the array contains `element`,
    /// between the range `start..=end` (inclusive).
    ///
    /// # Errors
    /// Returns [`MismatchedBounds::IndexOutOfBounds`] if either `start` or `end` `>= CAP`,
    /// or [`MismatchedBounds::MismatchedIndices`] if `start > end`.
    pub fn contains_between(
        &self,
        element: &T,
        start: usize,
        end: usize,
    ) -> Result<bool, MismatchedBounds> {
        is![start >= CAP; return Err(MismatchedBounds::IndexOutOfBounds(Some(start)))];
        is![end >= CAP; return Err(MismatchedBounds::IndexOutOfBounds(Some(end)))];
        is![start > end; return Err(MismatchedBounds::MismatchedIndices)];
        Ok(self.iter().skip(start).take(end - start + 1).any(|n| n == element))
    }

    /// Finds the index of the first occurrence of `element` in the array.
    ///
    /// # Example
    /// ```
    /// # use devela::{Array, ElementNotFound};
    /// let a = Array::<_, 5>::new([5, 78, 42, 33, 9]);
    /// assert_eq![a.find_index(&9), Ok(4)];
    /// assert_eq![a.find_index(&8), Err(ElementNotFound)];
    /// ```
    /// # Errors
    /// Returns [`ElementNotFound`] if the `element` can't be found.
    pub fn find_index(&self, element: &T) -> Result<usize, ElementNotFound> {
        self.iter()
            .enumerate()
            .find_map(|(i, n)| (n == element).then_some(i))
            .ok_or(ElementNotFound)
    }
}

/// # `Option<T>` methods
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

    /// Fills all `None` elements of the array with the given cloned `value`.
    pub fn fill_none(&mut self, value: T)
    where
        T: Clone,
    {
        self.iter_mut().filter(|opt| opt.is_none()).for_each(|opt| *opt = Some(value.clone()));
    }

    /// Returns the index of the first `None` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    pub fn first_none(&self) -> Result<usize, ElementNotFound> {
        self.iter().position(|opt| opt.is_none()).ok_or(ElementNotFound)
    }
    /// Returns some reference to the first `None` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    pub fn first_none_ref(&self) -> Result<&Option<T>, ElementNotFound> {
        self.iter().find(|opt| opt.is_none()).ok_or(ElementNotFound)
    }
    /// Returns some exclusive reference to the first `None` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    pub fn first_none_mut(&mut self) -> Result<&mut Option<T>, ElementNotFound> {
        self.iter_mut().find(|opt| opt.is_none()).ok_or(ElementNotFound)
    }

    /// Returns the index of the first `Some` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    pub fn first_some(&self) -> Result<usize, ElementNotFound> {
        self.iter().position(|opt| opt.is_some()).ok_or(ElementNotFound)
    }
    /// Returns some reference to the first `Some` element
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    pub fn first_some_ref(&self) -> Result<&Option<T>, ElementNotFound> {
        self.iter().find(|opt| opt.is_some()).ok_or(ElementNotFound)
    }
    /// Returns some exclusive reference to the first `Some` element.
    ///
    /// # Errors
    /// Returns [`ElementNotFound`] if the array is full.
    pub fn first_some_mut(&mut self) -> Result<&mut Option<T>, ElementNotFound> {
        self.iter_mut().find(|opt| opt.is_some()).ok_or(ElementNotFound)
    }
}
