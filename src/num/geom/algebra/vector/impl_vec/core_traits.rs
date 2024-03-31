// devela::num::geom::algebra::vector::impl_vec::core_traits
//
//!
//

use crate::num::geom::VecVector;
use core::fmt;

/* Clone, Copy */

// T:Clone
impl<T: Clone> Clone for VecVector<T> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
        }
    }
}

/* Default, ConstDefault */

impl<T: Default> Default for VecVector<T> {
    /// Returns a `VecVector`, using the default value to fill the data.
    fn default() -> Self {
        Self {
            vec: Default::default(),
        }
    }
}

// T:Debug
impl<T: fmt::Debug> fmt::Debug for VecVector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VecVector").field("vec", &self.vec).finish()
    }
}

// T:PartialEq
impl<T: PartialEq> PartialEq for VecVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}
// T:Eq
impl<T: Eq> Eq for VecVector<T> {}
