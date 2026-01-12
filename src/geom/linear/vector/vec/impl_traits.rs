// devela::geom::linear::vector::vec::impl_traits
//
//!
//

use crate::VecVector;
use core::fmt;

/* Clone, Copy */

// T:Clone
impl<T: Clone> Clone for VecVector<T> {
    fn clone(&self) -> Self {
        Self { coords: self.coords.clone() }
    }
}

/* Default, ConstInit TODO */

impl<T: Default> Default for VecVector<T> {
    /// Returns a `VecVector`, using the default value to fill the data.
    fn default() -> Self {
        Self { coords: Default::default() }
    }
}

// T:Debug
impl<T: fmt::Debug> fmt::Debug for VecVector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VecVector").field("coords", &self.coords).finish()
    }
}

// T:PartialEq
impl<T: PartialEq> PartialEq for VecVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}
// T:Eq
impl<T: Eq> Eq for VecVector<T> {}
