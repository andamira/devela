// devela::num::geom::metric::extent::impls_core
//
//!
//

use crate::{ConstDefault, ExtArray, Extent, Hash, Hasher, Ordering, array_init};
use core::fmt;

impl<T: Clone, const D: usize> Clone for Extent<T, D> {
    fn clone(&self) -> Self {
        Self::new(self.size.clone())
    }
}
impl<T: Copy, const D: usize> Copy for Extent<T, D> {}

impl<T: Default, const D: usize> Default for Extent<T, D> {
    fn default() -> Self {
        Self::new(array_init![default [T; D], "safe_num", "unsafe_array"])
    }
}
impl<T: ConstDefault, const D: usize> ConstDefault for Extent<T, D> {
    const DEFAULT: Self = Self::new(array_init![const_default [T; D]]);
}

impl<T: fmt::Debug, const D: usize> fmt::Debug for Extent<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Extent").field(&self.size).finish()
    }
}
impl<T: fmt::Display, const D: usize> fmt::Display for Extent<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Extent {{ size: {} }}", self.size.fmt())
    }
}

impl<T: PartialEq, const D: usize> PartialEq for Extent<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}
impl<T: Eq, const D: usize> Eq for Extent<T, D> {}

impl<T: PartialOrd, const D: usize> PartialOrd for Extent<T, D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size.partial_cmp(&other.size)
    }
}
impl<T: Ord, const D: usize> Ord for Extent<T, D> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl<T: Hash, const D: usize> Hash for Extent<T, D> {
    fn hash<HR: Hasher>(&self, state: &mut HR) {
        self.size.hash(state);
    }
}
