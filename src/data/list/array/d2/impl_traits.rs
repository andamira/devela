// devela::data::list::array::d2::impl_traits
//
//! 2-dimensional array trait impls
//

#[cfg(feature = "alloc")]
use crate::Boxed;
use crate::{Array, Array2d, Bare, ConstInit, Storage};
use core::fmt;

/* Clone, Copy */

#[rustfmt::skip]
impl<T: Clone, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
Clone for Array2d<T, C, R, CR, RMAJ, S> where S::Stored<[T; CR]>: Clone {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}
#[rustfmt::skip]
impl<T: Copy, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
Copy for Array2d<T, C, R, CR, RMAJ, S> where S::Stored<[T; CR]>: Copy {}

// Debug
#[rustfmt::skip]
impl<T: fmt::Debug, const C: usize, const R: usize, const CR: usize, S: Storage, const RMAJ: bool>
fmt::Debug for Array2d<T, C, R, CR, RMAJ, S> where S::Stored<[T; CR]>: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Array2d")
            .field("T", &core::any::type_name::<T>())
            .field("S", &S::name())
            .field("C", &C).field("R", &R).field("CR", &CR).field("RMAJ", &RMAJ)
            .field("data", &self.data)
            .finish()
    }
}

/* PartialEq, Eq */

#[rustfmt::skip]
impl<T: PartialEq, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
PartialEq for Array2d<T, C, R, CR, RMAJ, S> where S::Stored<[T; CR]>: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.capacity() == other.capacity()
    }
}
#[rustfmt::skip]
impl<T: Eq, const C: usize, const R: usize, const CR: usize, const RMAJ: bool, S: Storage>
Eq for Array2d<T, C, R, CR, RMAJ, S> where S::Stored<[T; CR]>: Eq {}

/* Default, ConstInit */

// T: Default, S: Bare
impl<T: Default, const C: usize, const R: usize, const CR: usize, const RMAJ: bool> Default
    for Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    /// # Panics
    /// Panics if `C * R > usize::MAX` or if `C * R != CR`.
    fn default() -> Self {
        Self::panic_check_CR();
        Self { data: Array::<T, CR, Bare>::default() }
    }
}

// T: ConstInit, S: Bare
impl<T: ConstInit, const C: usize, const R: usize, const CR: usize, const RMAJ: bool> ConstInit
    for Array2d<T, C, R, CR, RMAJ, Bare>
{
    /// Returns an array, allocated in the stack,
    /// using the default value to fill the data.
    /// # Panics
    /// Panics if `C * R > usize::MAX` or if `C * R != CR`.
    const INIT: Self = {
        Self::panic_check_CR();
        Self { data: Array::<T, CR, Bare>::INIT }
    };
}

// T: Default, S: Boxed
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
impl<T: Default, const C: usize, const R: usize, const CR: usize, const RMAJ: bool> Default
    for Array2d<T, C, R, CR, RMAJ, Boxed>
{
    /// Returns an array, allocated in the heap,
    /// using the default value to fill the data.
    ///
    /// # Examples
    /// ```
    /// # use devela::{Boxed, Array2d};
    /// let g = Array2d::<String, 4, 4, {4 * 4}, true, Boxed>::default();
    /// ```
    fn default() -> Self {
        Self::panic_check_CR();
        Self { data: Array::<T, CR, Boxed>::default() }
    }
}
