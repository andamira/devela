// devela::data::collections::array::d2::methods::general
//
//! 2-dimensional array general methods
//

#[cfg(feature = "alloc")]
use crate::{
    _deps::alloc::vec::Vec,
    mem::{Box, Boxed},
};
use crate::{
    code::iif,
    data::{
        error::{DataError, DataResult as Result},
        {Array, Array2d},
    },
    mem::{Bare, Storage},
    result::Mismatch,
};
use DataError::{MismatchedLength, Overflow};

/* constructors */

// S:Bare + T:Clone
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
        Ok(Self {
            array: Array::<T, Bare, CR>::with_cloned(element),
        })
    }
}
// S:Bare + T:Copy
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
    /// # use devela::all::{DataResult, Array2d};
    /// const GRID: DataResult<Array2d::<char, 4, 4, {4 * 4}>> = Array2d::with_copied('.');
    /// assert![GRID.is_ok()];
    /// ```
    pub const fn with_copied(element: T) -> Result<Self> {
        match Self::check_CR() {
            Ok(_) => Ok(Self {
                array: Array::<T, Bare, CR>::with_copied(element),
            }),
            Err(e) => Err(e),
        }
    }
}

// S:Boxed + T:Clone
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
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
    /// # use devela::all::{Boxed, Array2d};
    /// let g = Array2d::<_, 4, 4, {4 * 4}, Boxed>::with_cloned(String::from("·"));
    /// ```
    pub fn with_cloned(element: T) -> Result<Self> {
        Self::check_CR()?;
        Ok(Self {
            array: Array::<T, Boxed, CR>::with_cloned(element),
        })
    }
}

// Order-independent
#[rustfmt::skip]
impl<T, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
    Array2d<T, C, R, CR, RMAJ, S>
{
    /* general queries */

    /// Returns the total length of the array, equals `C * R`.
    #[inline] #[must_use]
    pub const fn len(&self) -> usize { CR }

    /// Returns the length of the C axis.
    /// (AKA width, number of columns or row length).
    #[inline] #[must_use] pub const fn x_len(&self) -> usize { C }

    /// Returns the length of the R axis
    /// (AKA height, number of rows or column length).
    #[inline] #[must_use] pub const fn y_len(&self) -> usize { C }

    /// Returns `true` if the stack is empty (has 0 length).
    /// # Examples
    /// ```
    /// # use devela::all::Array2d;
    /// let g1 = Array2d::<i32, 3, 3, 9>::default();
    /// assert![!g1.is_empty()];
    ///
    /// let g2 = Array2d::<i32, 0, 0, 0>::default();
    /// assert![g2.is_empty()];
    /// ```
    #[inline] #[must_use]
    pub const fn is_empty(&self) -> bool { CR == 0 }

    /// Checks the geometry of the columns, rows and their product length.
    /// # Errors
    /// Returns [`Overflow`] if `C * R > usize::MAX`
    /// or [`MismatchedLength`] if `C * R != CR`.
    #[inline] #[allow(non_snake_case)]
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
    #[inline] #[allow(non_snake_case)]
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
    // /// # use devela::all::Array2d;
    // /// let g = Array2d::from([1, 2, 3]);
    // /// assert_eq![s.as_slice(), &[1, 2, 3]];
    // /// ```
    #[inline] #[must_use]
    pub fn as_slice(&self) -> &[T] { self.array.as_slice() }

    /// Returns the stack as an exclusive slice.
    // /// # Examples
    // /// ```
    // /// # use devela::all::Array2d;
    // /// let mut g = Array2d::<_, (), 2, 2, 4>::from([1, 2, 3]);
    // /// assert_eq![s.as_mut_slice(), &mut [1, 2, 3]];
    // /// ```
    #[inline] #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] { self.array.as_mut_slice() }
}

// S:Bare
#[rustfmt::skip]
impl<T, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns the inner [`BareBox`]ed primitive array.
    #[inline] #[must_use]
    pub fn into_array(self) -> [T; CR] { self.array.into_array() }
}

// S:Bare, T:Copy
#[rustfmt::skip]
impl<T: Copy, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns the inner [`BareBox`]ed primitive array in compile-time.
    #[inline] #[must_use]
    pub const fn into_array_const(self) -> [T; CR] { self.array.into_array_const() }
}

// S:Boxed
#[rustfmt::skip]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    Array2d<T, C, R, CR, RMAJ, Boxed>
{
    /// Returns the inner [`Box`]ed primitive array.
    #[inline] #[must_use]
    pub fn into_array(self) -> Box<[T; CR]> { self.array.into_array() }

    /// Returns the inner [`Box`]ed primitive array as a slice.
    #[inline] #[must_use]
    pub fn into_slice(self) -> Box<[T]> { self.array.into_slice() }

    /// Returns the inner [`Box`]ed primitive array as a `Vec`.
    #[inline] #[must_use]
    pub fn into_vec(self) -> Vec<T> { self.array.into_vec() }
}

// T: Clone
#[rustfmt::skip]
impl<T: Clone, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
    Array2d<T, C, R, CR, RMAJ, S>
{
    /// Fills all elements of the grid with the given `element`.
    #[inline]
    pub fn fill(&mut self, element: T) { self.array.fill(element) }
}

// T:PartialEq
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
    // /// # use devela::all::Array2d;
    // /// let a = Array2d::<_, 5, 5, 25>::new([5, 78, 42, 33, 9]);
    // /// assert![a.contains(&9)];
    // /// assert![!a.contains(&8)];
    // /// ```
    #[inline]
    #[must_use]
    pub fn contains(&self, element: &T) -> bool {
        self.array.iter().any(|n| n == element)
    }
}
