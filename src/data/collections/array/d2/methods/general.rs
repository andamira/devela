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

// `S:Bare + T:Clone`
impl<T: Clone, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    Array2d<T, Bare, X, Y, LEN, XMAJ>
{
    /// Returns a 2-dimensional grid, allocated in the stack,
    /// using `element` to fill the remaining free data.
    /// # Errors
    /// Returns [`Overflow`] if `X * Y > usize::MAX`
    /// or [`MismatchedLength`] if `X * Y != LEN`.
    /// # Examples
    /// ```
    /// # use devela::data::Array2d;
    /// let g = Array2d::<_, (), 4, 4, {4 * 4}>::with_cloned('.');
    /// ```
    pub fn with_cloned(element: T) -> Result<Self> {
        Self::check_CRLEN()?;
        Ok(Self {
            array: Array::<T, Bare, LEN>::with_cloned(element),
        })
    }
}
// `S:Bare + T:Copy`
impl<T: Copy, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    Array2d<T, Bare, X, Y, LEN, XMAJ>
{
    /// Returns a 2-dimensional grid, allocated in the stack,
    /// using `element` to fill the remaining free data.
    /// # Errors
    /// Returns [`Overflow`] if `X * Y > usize::MAX`
    /// or [`MismatchedLength`] if `X * Y != LEN`.
    /// # Examples
    /// ```
    /// # use devela::all::{DataResult, Array2d};
    /// const GRID: DataResult<Array2d::<char, (), 4, 4, {4 * 4}>> = Array2d::with_copied('.');
    /// assert![GRID.is_ok()];
    /// ```
    pub const fn with_copied(element: T) -> Result<Self> {
        match Self::check_CRLEN() {
            Ok(_) => Ok(Self {
                array: Array::<T, Bare, LEN>::with_copied(element),
            }),
            Err(e) => Err(e),
        }
    }
}

// S:Boxed + T:Clone
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
impl<T: Clone, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    Array2d<T, Boxed, X, Y, LEN, XMAJ>
{
    /// Returns a 2-dimensional grid, allocated in the heap,
    /// using `element` to fill the remaining free data.
    /// # Errors
    /// Returns [`Overflow`] if `X * Y > usize::MAX`
    /// or [`MismatchedLength`] if `X * Y != LEN`.
    /// # Examples
    /// ```
    /// # use devela::all::{Boxed, Array2d};
    /// let g = Array2d::<_, Boxed, 4, 4, {4 * 4}>::with_cloned(String::from("·"));
    /// ```
    pub fn with_cloned(element: T) -> Result<Self> {
        Self::check_CRLEN()?;
        Ok(Self {
            array: Array::<T, Boxed, LEN>::with_cloned(element),
        })
    }
}

// Order-independent
#[rustfmt::skip]
impl<T, S: Storage, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    Array2d<T, S, X, Y, LEN, XMAJ>
{
    /* general queries */

    /// Returns the total length of the array, equals `X * Y`.
    #[inline] #[must_use]
    pub const fn len(&self) -> usize { LEN }

    /// Returns the length of the X axis.
    /// (AKA width, number of columns or row length).
    #[inline] #[must_use] pub const fn x_len(&self) -> usize { X }

    /// Returns the length of the Y axis
    /// (AKA height, number of rows or column length).
    #[inline] #[must_use] pub const fn y_len(&self) -> usize { X }

    /// Returns `true` if the stack is empty (has 0 length).
    /// # Examples
    /// ```
    /// # use devela::all::Array2d;
    /// let g1 = Array2d::<i32, (), 3, 3, 9>::default();
    /// assert![!g1.is_empty()];
    ///
    /// let g2 = Array2d::<i32, (), 0, 0, 0>::default();
    /// assert![g2.is_empty()];
    /// ```
    #[inline] #[must_use]
    pub const fn is_empty(&self) -> bool { LEN == 0 }

    /// Checks the geometry of the columns, rows and their product length.
    /// # Errors
    /// Returns [`Overflow`] if `X * Y > usize::MAX`
    /// or [`MismatchedLength`] if `X * Y != LEN`.
    #[inline] #[allow(non_snake_case)]
    pub(crate) const fn check_CRLEN() -> Result<()> {
        if let Some(len) = X.checked_mul(Y) {
            if len == LEN {
                Ok(())
            } else {
                Err(MismatchedLength(Mismatch { need: LEN, have: len }))
            }
        } else {
            Err(Overflow)
        }
    }

    /// Checks the geometry of the columns, rows and their product length.
    /// # Panics
    /// Panics if `X * Y > usize::MAX` or if `X * Y != LEN`.
    #[inline] #[allow(non_snake_case)]
    pub(crate) const fn panic_check_CRLEN() {
        if let Some(len) = X.checked_mul(Y) {
            if len != LEN {
                panic![concat![ "Array2d Mismatch: Columns * Rows != LEN: ",
                    stringify!(X), " * ", stringify!(Y), " != ", stringify!(LEN) ]];
            }
        } else {
            panic![concat![ "Array2d overflow: Columns * Rows (",
                stringify!(Y), " * ", stringify!(X), " > usize::MAX)" ]];
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
impl<T, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    Array2d<T, Bare, X, Y, LEN, XMAJ>
{
    /// Returns the inner [`BareBox`]ed primitive array.
    #[inline] #[must_use]
    pub fn into_array(self) -> [T; LEN] { self.array.into_array() }
}

// S:Bare, T:Copy
#[rustfmt::skip]
impl<T: Copy, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    Array2d<T, Bare, X, Y, LEN, XMAJ>
{
    /// Returns the inner [`BareBox`]ed primitive array in compile-time.
    #[inline] #[must_use]
    pub const fn into_array_const(self) -> [T; LEN] { self.array.into_array_const() }
}

// S:Boxed
#[rustfmt::skip]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    Array2d<T, Boxed, X, Y, LEN, XMAJ>
{
    /// Returns the inner [`Box`]ed primitive array.
    #[inline] #[must_use]
    pub fn into_array(self) -> Box<[T; LEN]> { self.array.into_array() }

    /// Returns the inner [`Box`]ed primitive array as a slice.
    #[inline] #[must_use]
    pub fn into_slice(self) -> Box<[T]> { self.array.into_slice() }

    /// Returns the inner [`Box`]ed primitive array as a `Vec`.
    #[inline] #[must_use]
    pub fn into_vec(self) -> Vec<T> { self.array.into_vec() }
}

// T: Clone
#[rustfmt::skip]
impl<T: Clone, S: Storage, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    Array2d<T, S, X, Y, LEN, XMAJ>
{
    /// Fills all elements of the grid with the given `element`.
    #[inline]
    pub fn fill(&mut self, element: T) { self.array.fill(element) }
}

// T:PartialEq
impl<
        T: PartialEq,
        S: Storage,
        const X: usize,
        const Y: usize,
        const LEN: usize,
        const XMAJ: bool,
    > Array2d<T, S, X, Y, LEN, XMAJ>
{
    /// Returns true if the array contains `element`.
    /// # Examples
    // /// ```
    // /// # use devela::all::Array2d;
    // /// let a = Array2d::<_, (), 5>::new([5, 78, 42, 33, 9]);
    // /// assert![a.contains(&9)];
    // /// assert![!a.contains(&8)];
    // /// ```
    #[inline]
    #[must_use]
    pub fn contains(&self, element: &T) -> bool {
        self.array.iter().any(|n| n == element)
    }
}
