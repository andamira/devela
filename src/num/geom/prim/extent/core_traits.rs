// devela::num::geom::prim::extent::impls_core
//
//!
//

use super::Extent;
use crate::{
    code::ConstDefault,
    data::{array_init, ExtArray},
};
use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

impl<T: Clone, const D: usize> Clone for Extent<T, D> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.extent.clone())
    }
}
impl<T: Copy, const D: usize> Copy for Extent<T, D> {}

impl<T: Default, const D: usize> Default for Extent<T, D> {
    #[inline]
    fn default() -> Self {
        Self::new(array_init![default [T; D], "safe_num", "unsafe_array"])
    }
}
impl<T: ConstDefault, const D: usize> ConstDefault for Extent<T, D> {
    const DEFAULT: Self = Self::new(array_init![const_default [T; D]]);
}

impl<T: fmt::Debug, const D: usize> fmt::Debug for Extent<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Extent").field(&self.extent).finish()
    }
}
impl<T: fmt::Display, const D: usize> fmt::Display for Extent<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Extent {{ extent: {} }}", self.extent.fmt())
    }
}

impl<T: PartialEq, const D: usize> PartialEq for Extent<T, D> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.extent == other.extent
    }
}
impl<T: Eq, const D: usize> Eq for Extent<T, D> {}

impl<T: PartialOrd, const D: usize> PartialOrd for Extent<T, D> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.extent.partial_cmp(&other.extent)
    }
}
impl<T: Ord, const D: usize> Ord for Extent<T, D> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.extent.cmp(&other.extent)
    }
}

impl<T: Hash, const D: usize> Hash for Extent<T, D> {
    #[inline]
    fn hash<HR: Hasher>(&self, state: &mut HR) {
        self.extent.hash(state);
    }
}
