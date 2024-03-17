// devela::fig::prim::point::core_traits
//
//!
//

use super::Point;
use crate::{
    code::ConstDefault,
    data::{array_init, ExtArray},
};
use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

impl<T: Clone, const D: usize> Clone for Point<T, D> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.coords.array.clone())
    }
}
impl<T: Copy, const D: usize> Copy for Point<T, D> {}

impl<T: Default, const D: usize> Default for Point<T, D> {
    #[inline]
    fn default() -> Self {
        Self::new(array_init![default [T; D], "safe_fig", "unsafe_array"])
    }
}
impl<T: ConstDefault, const D: usize> ConstDefault for Point<T, D> {
    const DEFAULT: Self = Self::new(array_init![const_default [T; D]]);
}

impl<T: fmt::Debug, const D: usize> fmt::Debug for Point<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Point").field(&self.coords).finish()
    }
}
impl<T: fmt::Display, const D: usize> fmt::Display for Point<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ coords: {} }}", self.coords.array.fmt())
    }
}

impl<T: PartialEq, const D: usize> PartialEq for Point<T, D> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}
impl<T: Eq, const D: usize> Eq for Point<T, D> {}

impl<T: PartialOrd, const D: usize> PartialOrd for Point<T, D> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.coords.array.partial_cmp(&other.coords.array)
    }
}
impl<T: Ord, const D: usize> Ord for Point<T, D> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.coords.array.cmp(&other.coords.array)
    }
}

impl<T: Hash, const D: usize> Hash for Point<T, D> {
    #[inline]
    fn hash<HR: Hasher>(&self, state: &mut HR) {
        self.coords.hash(state);
    }
}
