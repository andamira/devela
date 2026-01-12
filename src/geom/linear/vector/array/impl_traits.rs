// devela::geom::linear::vector::array:impl_traits
//
//!
//

use crate::{ConstInit, Hash, Hasher, Vector, init_array};
use ::core::fmt;

/* Clone, Copy */

// T:Clone
impl<T: Clone, const D: usize> Clone for Vector<T, D> {
    fn clone(&self) -> Self {
        Self { coords: self.coords.clone() }
    }
}

// T:Copy
impl<T: Copy, const D: usize> Copy for Vector<T, D> {}

/* Default, ConstInit */

impl<T: Default, const D: usize> Default for Vector<T, D> {
    /// Returns a `Vector`, allocated in the stack,
    /// using the default value to fill the data.
    fn default() -> Self {
        Self::new(init_array![default [T; D], "safe_num", "unsafe_array"])
    }
}

// S:Bare + T:ConstInit
impl<T: ConstInit, const D: usize> ConstInit for Vector<T, D> {
    /// Returns a Vector, allocated in the stack,
    /// using the default value to fill the data.
    const INIT: Self = Self::new(init_array![INIT in ConstInit [T; D]]);
}

// T:Debug
impl<T: fmt::Debug, const D: usize> fmt::Debug for Vector<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector").field("D", &D).field("coords", &self.coords).finish()
    }
}

// T:PartialEq
impl<T: PartialEq, const D: usize> PartialEq for Vector<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}
// T:Eq
impl<T: Eq, const D: usize> Eq for Vector<T, D> {}

impl<T: Hash, const D: usize> Hash for Vector<T, D> {
    fn hash<HR: Hasher>(&self, state: &mut HR) {
        self.coords.hash(state);
    }
}
