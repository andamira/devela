// devela::data::list::array::d2::methods::general
//
//! 2-dimensional array general methods.
//

use crate::{
    Array, Array2d, Bare,
    MismatchedBounds::{self, IndexOutOfBounds},
    Storage,
};
#[cfg(doc)]
use crate::{BareBox, MismatchedBounds::CapacityMismatch};
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
    /// Returns [`IndexOutOfBounds`] if `C * R` is not representable as a `usize`,
    /// or [`CapacityMismatch`] if `C * R != CR`.
    /// # Examples
    /// ```
    /// # use devela::Array2d;
    /// let g = Array2d::<_, 4, 4, {4 * 4}>::with_cloned('.');
    /// ```
    pub fn with_cloned(element: T) -> Result<Self, MismatchedBounds> {
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
    /// Returns [`IndexOutOfBounds`] if `C * R` is not representable as a `usize`,
    /// or [`CapacityMismatch`] if `C * R != CR`.
    /// # Examples
    /// ```
    /// # use devela::{Array2d, MismatchedBounds};
    /// const GRID: Result<Array2d::<char, 4, 4, {4 * 4}>, MismatchedBounds>
    ///     = Array2d::with_copied('.');
    /// assert![GRID.is_ok()];
    /// ```
    pub const fn with_copied(element: T) -> Result<Self, MismatchedBounds> {
        match Self::check_CR() {
            Ok(()) => Ok(Self { data: Array::<T, CR, Bare>::with_copied(element) }),
            Err(e) => Err(e),
        }
    }
}

// T: Clone, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<T: Clone, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Boxed>
{
    /// Returns a 2-dimensional grid, allocated in the heap,
    /// using `element` to fill the remaining free data.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if `C * R` is not representable as a `usize`,
    /// or [`CapacityMismatch`] if `C * R != CR`.
    /// # Examples
    /// ```
    /// # use devela::{Boxed, Array2d};
    /// let g = Array2d::<_, 4, 4, {4 * 4}, true, Boxed>::with_cloned(String::from("·"));
    /// ```
    pub fn with_cloned(element: T) -> Result<Self, MismatchedBounds> {
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

    /// Returns the total capacity of the array, equals `CR == C * R`.
    #[must_use] pub const fn capacity(&self) -> usize { CR }

    /// Returns the capacity of a column, equivalent to [`num_rows`][Self::num_rows] == `R`.
    #[must_use] pub const fn cap_col(&self) -> usize { R }

    /// Returns the capacity of a row, equivalent to [`num_cols`][Self::num_cols] == `C`.
    #[must_use] pub const fn cap_row(&self) -> usize { C }

    /// Returns the number of columns, equivalent to [`cap_row`][Self::cap_row] == `C`.
    #[must_use] pub const fn num_cols(&self) -> usize { C }

    /// Returns the number of rows, equivalent to [`cap_col`][Self::cap_col] == `R`.
    #[must_use] pub const fn num_rows(&self) -> usize { R }

    /// Checks the geometry of the columns, rows and their product length.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if `C * R` is not representable as a `usize`,
    /// or [`CapacityMismatch`] if `C * R != CR`.
    #[allow(non_snake_case)]
    pub(crate) const fn check_CR() -> Result<(), MismatchedBounds> {
        match C.checked_mul(R) {
            Some(len) => {
                if len == CR {
                    Ok(())
                } else if len > CR {
                    let err = crate::CapacityMismatch::too_large(len, CR);
                    Err(MismatchedBounds::from_capacity_mismatch(err))
                } else {
                    let err = crate::CapacityMismatch::too_small(len, CR);
                    Err(MismatchedBounds::from_capacity_mismatch(err))
                }
            }
            None => Err(IndexOutOfBounds(None)),
        }
    }

    /// Checks the geometry of the columns, rows and their product length.
    /// # Panics
    /// Panics if `C * R` is not representable as a `usize`, or if `C * R != CR`.
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

/* Order-dependent */

#[rustfmt::skip]
impl<T, const C: usize, const R: usize, const CR: usize, S: Storage> Array2d<T, C, R, CR, true, S> {
    /// Returns the capacity per item in the major dimension based on layout (`RMAJ`).
    /// For row-major, this is the number of columns. For column-major, this is the number of rows.
    #[must_use] pub const fn cap_major(&self) -> usize { C }

    /// Returns the capacity per item in the minor dimension based on layout (`RMAJ`).
    /// For row-major, this is the number of rows. For column-major, this is the number of columns.
    #[must_use] pub const fn cap_minor(&self) -> usize { R }

    /// Returns the number of items in the major dimension based on layout (`RMAJ`).
    /// For row-major, this is the number of rows. For column-major, this is the number of columns.
    #[must_use] pub const fn num_major(&self) -> usize { R }

    /// Returns the number of items in the minor dimension based on layout (`RMAJ`).
    /// For row-major, this is the number of columns. For column-major, this is the number of rows.
    #[must_use] pub const fn num_minor(&self) -> usize { C }
}

#[rustfmt::skip]
impl<T, const C: usize, const R: usize, const CR: usize, S: Storage> Array2d<T, C, R, CR, false, S> {
    /// Returns the capacity per item in the major dimension based on layout (`RMAJ`).
    /// For row-major, this is the number of columns. For column-major, this is the number of rows.
    #[must_use] pub const fn cap_major(&self) -> usize { R }

    /// Returns the capacity per item in the minor dimension based on layout (`RMAJ`).
    /// For row-major, this is the number of rows. For column-major, this is the number of columns.
    #[must_use] pub const fn cap_minor(&self) -> usize { C }

    /// Returns the number of items in the major dimension based on layout (`RMAJ`).
    /// For row-major, this is the number of rows. For column-major, this is the number of columns.
    #[must_use] pub const fn num_major(&self) -> usize { C }

    /// Returns the number of items in the minor dimension based on layout (`RMAJ`).
    /// For row-major, this is the number of columns. For column-major, this is the number of rows.
    #[must_use] pub const fn num_minor(&self) -> usize { R }
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
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
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
impl<T: PartialEq, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
    Array2d<T, C, R, CR, RMAJ, S>
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
