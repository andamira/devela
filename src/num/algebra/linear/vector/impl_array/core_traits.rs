// devela::num::algebra::linear::vector::impl_array::core_traits
//
//!
//

use crate::{code::ConstDefault, data::array_init, num::algebra::linear::Vector};
use core::{
    fmt,
    hash::{Hash, Hasher},
};

/* Clone, Copy */

// T:Clone
impl<T: Clone, const D: usize> Clone for Vector<T, D> {
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
        }
    }
}

// T:Copy
impl<T: Copy, const D: usize> Copy for Vector<T, D> {}

/* Default, ConstDefault */

impl<T: Default, const D: usize> Default for Vector<T, D> {
    /// Returns a `Vector`, allocated in the stack,
    /// using the default value to fill the data.
    fn default() -> Self {
        let array = array_init![default [T; D], "safe_num", "unsafe_array"];
        Self { array }
    }
}

// S:Bare + T:ConstDefault
impl<T: ConstDefault, const D: usize> ConstDefault for Vector<T, D> {
    /// Returns a Vector, allocated in the stack,
    /// using the default value to fill the data.
    const DEFAULT: Self = {
        let array = array_init![const_default [T; D]];
        Self { array }
    };
}

// T:Debug
impl<T: fmt::Debug, const D: usize> fmt::Debug for Vector<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector")
            .field("D", &D)
            .field("array", &self.array)
            .finish()
    }
}

// T:PartialEq
impl<T: PartialEq, const D: usize> PartialEq for Vector<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
    }
}
// T:Eq
impl<T: Eq, const D: usize> Eq for Vector<T, D> {}

impl<T: Hash, const D: usize> Hash for Vector<T, D> {
    #[inline]
    fn hash<HR: Hasher>(&self, state: &mut HR) {
        self.array.hash(state);
    }
}
