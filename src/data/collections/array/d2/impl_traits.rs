// devela::data::collections::array::d2::impl_traits
//
//! 2-dimensional array trait impls
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    code::ConstDefault,
    data::{Array, DataCollection, DataResult as Result, Array2d, /* DataArray,*/ /*Array2dIter*/ },
    mem::{Bare, Storage},
};
use core::{cmp::Ordering, fmt};


/* Default */

// S:Bare + T:Default
impl<T: Default, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool> Default
    for Array2d<T, Bare, X, Y, LEN, XMAJ>
{
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    /// # Panics
    /// Panics if `X * Y > usize::MAX` or if `X * Y != LEN`.
    fn default() -> Self {
        Self::panic_check_CRLEN();
        Self {
            array: Array::<T, Bare, LEN>::default(),
        }
    }
}

// S:Bare + T:ConstDefault
impl<T: ConstDefault, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool>
    ConstDefault for Array2d<T, Bare, X, Y, LEN, XMAJ>
{
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    /// # Panics
    /// Panics if `X * Y > usize::MAX` or if `X * Y != LEN`.
    const DEFAULT: Self = {
        Self::panic_check_CRLEN();
        Self {
            array: Array::<T, Bare, LEN>::DEFAULT,
        }
    };
}

// S:Boxed + T:Default
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Default, const X: usize, const Y: usize, const LEN: usize, const XMAJ: bool> Default
    for Array2d<T, Boxed, X, Y, LEN, XMAJ>
{
    /// Returns an array, allocated in the heap,
    /// using the default value to fill the data.
    ///
    /// # Examples
    /// ```
    /// # use devela::all::{Boxed, Array2d};
    /// let g = Array2d::<String, Boxed, 4, 4, {4 * 4}>::default();
    /// ```
    fn default() -> Self {
        Self::panic_check_CRLEN();
        Self {
            array: Array::<T, Boxed, LEN>::default(),
        }
    }
}

// T: Debug row-major
impl<T: fmt::Debug, S: Storage, const X: usize, const Y: usize, const LEN: usize> fmt::Debug
    for Array2d<T, S, X, Y, LEN, true>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array2d {{ ")?;
        write!(f, "T: {}, ", core::any::type_name::<T>())?; // ?
        write!(f, "S: {}, ", S::name())?;
        write!(f, "X:{X}, Y:{Y} = LEN:{LEN}, ")?;
        write!(f, "row-major")?;

        // TODO: first 6 elements
        // TODO: last 6 elements

        write!(f, " }}")
    }
}
// col-major
impl<T: fmt::Debug, S: Storage, const X: usize, const Y: usize, const LEN: usize> fmt::Debug
    for Array2d<T, S, X, Y, LEN, false>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array2d {{ ")?;
        write!(f, "T: {}, ", core::any::type_name::<T>())?; // ?
        write!(f, "S: {}, ", S::name())?;
        write!(f, "X:{X}, Y:{Y} = LEN:{LEN}, ")?;
        write!(f, "col-major")?;

        // TODO: first 6 elements
        // TODO: last 6 elements

        write!(f, " }}")
    }
}
