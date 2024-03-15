// devela::fig::algebra::vector::impl::core_traits
//
//!
//

use crate::{code::ConstDefault, data::array_init, fig::Vector};
use core::{cmp::Ordering, fmt};

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
        let array = array_init![default [T; D], "safe_fig", "unsafe_array"];
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

// // T:PartialOrd
// impl<T: PartialOrd, const D: usize> PartialOrd for Vector<T, D>
// {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.iter().partial_cmp(other.iter())
//     }
// }
//
// // T:Ord
// impl<T: Ord, const D: usize> Ord for Vector<T, D>
// {
//     #[inline]
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.iter().cmp(other.iter())
//     }
// }
