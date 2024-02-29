// devela::data::collections::array::d2::impl_traits
//
//! 2-dimensional array trait impls
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::{
    code::ConstDefault,
    data::{
        Array, Array2d, /* DataArray,*/
        /*Array2dIter*/
        DataCollection, DataResult as Result,
    },
    mem::{Bare, Storage},
};
use core::{cmp::Ordering, fmt};

/* Clone, Copy */

// T:Clone
impl<T: Clone, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage> Clone
    for Array2d<T, C, R, CR, RMAJ, S>
where
    S::Stored<[T; CR]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
        }
    }
}

// T:Copy
impl<T: Copy, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage> Copy
    for Array2d<T, C, R, CR, RMAJ, S>
where
    S::Stored<[T; CR]>: Copy,
{
}

/* Default, ConstDefault */

// S:Bare + T:Default
impl<T: Default, const C: usize, const R: usize, const CR: usize, const RMAJ: bool> Default
    for Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    /// # Panics
    /// Panics if `C * R > usize::MAX` or if `C * R != CR`.
    fn default() -> Self {
        Self::panic_check_CR();
        Self {
            array: Array::<T, CR, Bare>::default(),
        }
    }
}

// S:Bare + T:ConstDefault
impl<T: ConstDefault, const C: usize, const R: usize, const CR: usize, const RMAJ: bool>
    ConstDefault for Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    /// # Panics
    /// Panics if `C * R > usize::MAX` or if `C * R != CR`.
    const DEFAULT: Self = {
        Self::panic_check_CR();
        Self {
            array: Array::<T, CR, Bare>::DEFAULT,
        }
    };
}

// S:Boxed + T:Default
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
impl<T: Default, const C: usize, const R: usize, const CR: usize, const RMAJ: bool> Default
    for Array2d<T, C, R, CR, RMAJ, Boxed>
{
    /// Returns an array, allocated in the heap,
    /// using the default value to fill the data.
    ///
    /// # Examples
    /// ```
    /// # use devela::all::{Boxed, Array2d};
    /// let g = Array2d::<String, 4, 4, {4 * 4}, true, Boxed>::default();
    /// ```
    fn default() -> Self {
        Self::panic_check_CR();
        Self {
            array: Array::<T, CR, Boxed>::default(),
        }
    }
}

// T:Debug, row-major
impl<T: fmt::Debug, const C: usize, const R: usize, const CR: usize, S: Storage> fmt::Debug
    for Array2d<T, C, R, CR, true, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array2d {{ ")?;
        write!(f, "T: {}, ", core::any::type_name::<T>())?; // ?
        write!(f, "S: {}, ", S::name())?;
        write!(f, "C:{C}, R:{R} = CR:{CR}, ")?;
        write!(f, "row-major")?;

        // TODO: first 6 elements
        // TODO: last 6 elements

        write!(f, " }}")
    }
}
// T:Debug, column-major
impl<T: fmt::Debug, const C: usize, const R: usize, const CR: usize, S: Storage> fmt::Debug
    for Array2d<T, C, R, CR, false, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array2d {{ ")?;
        write!(f, "T: {}, ", core::any::type_name::<T>())?; // ?
        write!(f, "S: {}, ", S::name())?;
        write!(f, "C:{C}, R:{R} = CR:{CR}, ")?;
        write!(f, "column-major")?;

        // TODO: first 6 elements
        // TODO: last 6 elements

        write!(f, " }}")
    }
}
