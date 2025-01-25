// devela::data::list::array::d1::methods::bare
//
//! 1-dimensional array *Bare* methods.
//
// TOC
// - constructors
// - methods
// - indexing methods (panicking)
// - Option<T> methods

use crate::{array_init, iif, Array, Bare, BareBox};

/// # *Bare* constructors
impl<T, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns a new [`Array`] allocated in the stack,
    /// from the given primitive `array` in compile-time.
    pub const fn new_bare(array: [T; CAP]) -> Self {
        Self { data: BareBox::new(array) }
    }

    /// Returns an array, allocated in the stack, filled with `element`, cloned.
    ///
    /// # Example
    /// ```
    /// # use devela::data::Array;
    /// let a = Array::<_, 16>::with_cloned(0);
    /// ```
    pub fn with_cloned(element: T) -> Self
    where
        T: Clone,
    {
        let data = BareBox::new(array_init!(clone [T; CAP], "safe_data", "unsafe_array", element));
        Self { data }
    }

    /// Returns an array, allocated in the stack, filled with `element`, copied, in compile-time.
    ///
    /// # Example
    /// ```
    /// # use devela::data::Array;
    /// const A: Array<i32, 16> = Array::with_copied(0);
    /// ```
    pub const fn with_copied(element: T) -> Self
    where
        T: Copy,
    {
        let data = BareBox::new([element; CAP]);
        Self { data }
    }
}

/// # *Bare* methods
impl<T, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns a slice containing the entire array in compile time.
    ///
    /// It allows to sidestep `Deref` coercion for indexing purposes.
    #[must_use]
    pub const fn as_bare_slice(&self) -> &[T] {
        self.data.as_ref() // const method on BareBox
    }

    /// Returns a slice containing the entire array in compile time.
    ///
    /// It allows to sidestep `Deref` coercion for indexing purposes.
    #[must_use]
    pub const fn as_bare_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut() // const method on BareBox
    }

    /// Returns the inner [`BareBox`]ed primitive array.
    #[must_use]
    pub fn into_array(self) -> [T; CAP] {
        self.data.into_inner()
    }

    /// Returns the inner [`BareBox`]ed primitive array in compile-time.
    #[must_use]
    pub const fn into_array_copy(self) -> [T; CAP]
    where
        T: Copy,
    {
        self.data.into_inner_copy()
    }
}

/// # *Bare* indexing methods (panicking)
impl<T, const CAP: usize> Array<T, CAP, Bare> {
    /// Returns a shared reference to the element at the given `index` in compile-time.
    ///
    /// # Panics
    /// Panics if the `index` is out of bounds in a non-const context.
    ///
    /// # Example
    /// ```
    /// # use devela::Array;
    /// const A: Array<i32, 4> = Array::new_bare([10, 20, 30, 40]);
    /// const VALUE: i32 = *A.get(2);
    /// assert_eq!(VALUE, 30);
    /// ```
    #[must_use]
    pub const fn get(&self, index: usize) -> &T {
        assert!(index < CAP, "Index out of bounds in const context");
        &self.as_bare_slice()[index]
    }

    /// Returns an exclusive reference to the element at the given `index` in compile-time.
    ///
    /// # Panics
    /// Panics if the `index` is out of bounds in a non-const context.
    ///
    /// # Example
    /// ```
    /// # use devela::Array;
    /// const fn modify_array() -> Array<i32, 4> {
    ///     let mut a = Array::new_bare([10, 20, 30, 40]);
    ///     *a.get_mut(2) = 50;
    ///     a
    /// }
    /// const A: Array<i32, 4> = modify_array();
    /// assert_eq!(A.get_mut(2), &50);
    /// ```
    #[must_use]
    pub const fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < CAP, "Index out of bounds in const context");
        &mut self.as_bare_mut_slice()[index]
    }
}

/// # *Bare* `Option<T>` methods
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
