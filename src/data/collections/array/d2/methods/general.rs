// devela::data::collections::array::d2::methods::general
//
//! 2-dimensional array general methods
//

#[cfg(doc)]
use crate::BareBox;
use crate::{
    Array, Array2d, Bare,
    DataError::{MismatchedLength, Overflow},
    DataResult as Result, Mismatch, Storage,
};
#[cfg(feature = "alloc")]
use crate::{Box, Boxed, Vec};

/* constructors */

// T: Clone, S: Bare
impl<T: Clone, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns a 2-dimensional grid, allocated in the stack,
    /// using `element` to fill the remaining free data.
    /// # Errors
    /// Returns [`Overflow`] if `C * R > usize::MAX`
    /// or [`MismatchedLength`] if `C * R != CR`.
    /// # Examples
    /// ```
    /// # use devela::data::Array2d;
    /// let g = Array2d::<_, 4, 4, {4 * 4}>::with_cloned('.');
    /// ```
    pub fn with_cloned(element: T) -> Result<Self> {
        Self::check_CR()?;
        Ok(Self { data: Array::<T, CR, Bare>::with_cloned(element) })
    }
}
// T: Copy, S: Bare
impl<T: Copy, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns a 2-dimensional grid, allocated in the stack,
    /// using `element` to fill the remaining free data.
    /// # Errors
    /// Returns [`Overflow`] if `C * R > usize::MAX`
    /// or [`MismatchedLength`] if `C * R != CR`.
    /// # Examples
    /// ```
    /// # use devela::{DataResult, Array2d};
    /// const GRID: DataResult<Array2d::<char, 4, 4, {4 * 4}>> = Array2d::with_copied('.');
    /// assert![GRID.is_ok()];
    /// ```
    pub const fn with_copied(element: T) -> Result<Self> {
        match Self::check_CR() {
            Ok(()) => Ok(Self { data: Array::<T, CR, Bare>::with_copied(element) }),
            Err(e) => Err(e),
        }
    }
}

// T: Clone, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Clone, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Boxed>
{
    /// Returns a 2-dimensional grid, allocated in the heap,
    /// using `element` to fill the remaining free data.
    /// # Errors
    /// Returns [`Overflow`] if `C * R > usize::MAX`
    /// or [`MismatchedLength`] if `C * R != CR`.
    /// # Examples
    /// ```
    /// # use devela::{Boxed, Array2d};
    /// let g = Array2d::<_, 4, 4, {4 * 4}, true, Boxed>::with_cloned(String::from("·"));
    /// ```
    pub fn with_cloned(element: T) -> Result<Self> {
        Self::check_CR()?;
        Ok(Self { data: Array::<T, CR, Boxed>::with_cloned(element) })
    }
}

// Order-independent
// T, S
#[rustfmt::skip]
impl<T, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
    Array2d<T, C, R, CR, RMAJ, S>
{
    /* general queries */

    /// Returns the total length of the array, equals `C * R`.
    #[must_use]
    pub const fn len(&self) -> usize { CR }

    /// Returns the length of the C axis.
    /// (AKA width, number of columns or row length).
    #[must_use] pub const fn x_len(&self) -> usize { C }

    /// Returns the length of the R axis
    /// (AKA height, number of rows or column length).
    #[must_use] pub const fn y_len(&self) -> usize { C }

    /// Returns `true` if the stack is empty (has 0 length).
    /// # Examples
    /// ```
    /// # use devela::Array2d;
    /// let g1 = Array2d::<i32, 3, 3, 9>::default();
    /// assert![!g1.is_empty()];
    ///
    /// let g2 = Array2d::<i32, 0, 0, 0>::default();
    /// assert![g2.is_empty()];
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool { CR == 0 }

    /// Checks the geometry of the columns, rows and their product length.
    /// # Errors
    /// Returns [`Overflow`] if `C * R > usize::MAX`
    /// or [`MismatchedLength`] if `C * R != CR`.
    #[allow(non_snake_case)]
    pub(crate) const fn check_CR() -> Result<()> {
        if let Some(len) = C.checked_mul(R) {
            if len == CR {
                Ok(())
            } else {
                Err(MismatchedLength(Mismatch { need: CR, have: len, info: "C * R != CR" }))
            }
        } else {
            Err(Overflow)
        }
    }

    /// Checks the geometry of the columns, rows and their product length.
    /// # Panics
    /// Panics if `C * R > usize::MAX` or if `C * R != CR`.
    #[allow(non_snake_case)]
    pub(crate) const fn panic_check_CR() {
        if let Some(len) = C.checked_mul(R) {
            if len != CR {
                panic![concat![ "Array2d Mismatch: C * R != CR: ",
                    stringify!(C), " * ", stringify!(R), " != ", stringify!(CR) ]];
            }
        } else {
            panic![concat![ "Array2d overflow: C * R (",
                stringify!(R), " * ", stringify!(C), " > usize::MAX)" ]];
        }
    }

    /* slices */

    /// Returns a slice containing the full grid.
    // /// # Examples
    // /// ```
    // /// # use devela::Array2d;
    // /// let g = Array2d::from([1, 2, 3]);
    // /// assert_eq![s.as_slice(), &[1, 2, 3]];
    // /// ```
    #[must_use]
    pub fn as_slice(&self) -> &[T] { self.data.as_slice() }

    /// Returns the stack as an exclusive slice.
    // /// # Examples
    // /// ```
    // /// # use devela::Array2d;
    // /// let mut g = Array2d::<_, (), 2, 2, 4>::from([1, 2, 3]);
    // /// assert_eq![s.as_mut_slice(), &mut [1, 2, 3]];
    // /// ```
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] { self.data.as_mut_slice() }
}

// T, S: Bare
#[rustfmt::skip]
impl<T, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns the inner [`BareBox`]ed primitive array.
    #[must_use]
    pub fn into_array(self) -> [T; CR] { self.data.into_array() }
}

// T: Copy, S: Bare
#[rustfmt::skip]
impl<T: Copy, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns the inner [`BareBox`]ed primitive array in compile-time.
    #[must_use]
    pub const fn into_array_copy(self) -> [T; CR] { self.data.into_array_copy() }
}

// T, S: Boxed
#[rustfmt::skip]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Boxed>
{
    /// Returns the inner [`Box`]ed primitive array.
    #[must_use]
    pub fn into_array(self) -> Box<[T; CR]> { self.data.into_array() }

    /// Returns the inner [`Box`]ed primitive array as a slice.
    #[must_use]
    pub fn into_slice(self) -> Box<[T]> { self.data.into_slice() }

    /// Returns the inner [`Box`]ed primitive array as a `Vec`.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> { self.data.into_vec() }
}

// T: Clone, S
#[rustfmt::skip]
impl<T: Clone, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
    Array2d<T, C, R, CR, RMAJ, S>
{
    /// Fills all elements of the grid with the given `element`.
    pub fn fill(&mut self, element: T) { self.data.fill(element) }
}

// T: PartialEq, S
impl<
        T: PartialEq,
        const C: usize,
        const R: usize,
        const CR: usize,
        const RMAJ: bool,
        S: Storage,
    > Array2d<T, C, R, CR, RMAJ, S>
{
    /// Returns true if the array contains `element`.
    /// # Examples
    // /// ```
    // /// # use devela::Array2d;
    // /// let a = Array2d::<_, 5, 5, 25>::new([5, 78, 42, 33, 9]);
    // /// assert![a.contains(&9)];
    // /// assert![!a.contains(&8)];
    // /// ```
    #[must_use]
    pub fn contains(&self, element: &T) -> bool {
        self.data.iter().any(|n| n == element)
    }
}
