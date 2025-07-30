// devela::num::geom::metric::extent::impl_traits
//
//!
//

use crate::{ConstDefault, ExtArray, Extent, Hash, Hasher, Ordering, array_init};
use core::fmt;

impl<T: Clone, const D: usize> Clone for Extent<T, D> {
    fn clone(&self) -> Self {
        Self::new(self.dim.clone())
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
        f.debug_struct("Extent").field("dim", &self.dim).finish()
    }
}
impl<T: fmt::Display, const D: usize> fmt::Display for Extent<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Extent {{ dim: {} }}", self.dim.fmt()) // IMPROVE
    }
}

impl<T: PartialEq, const D: usize> PartialEq for Extent<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.dim == other.dim
    }
}
impl<T: Eq, const D: usize> Eq for Extent<T, D> {}

impl<T: PartialOrd, const D: usize> PartialOrd for Extent<T, D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dim.partial_cmp(&other.dim)
    }
}
impl<T: Ord, const D: usize> Ord for Extent<T, D> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dim.cmp(&other.dim)
    }
}

impl<T: Hash, const D: usize> Hash for Extent<T, D> {
    fn hash<HR: Hasher>(&self, state: &mut HR) {
        self.dim.hash(state);
    }
}

crate::items! {
    impl<T, const D: usize> From<[T; D]> for Extent<T, D> {
        fn from(dim: [T; D]) -> Self { Self { dim } }
    }
    impl<T> From<(T, T)> for Extent<T, 2> {
        fn from(dim: (T, T)) -> Self { Self { dim: [dim.0, dim.1] } }
    }
    impl<T> From<(T, T, T)> for Extent<T, 3> {
        fn from(dim: (T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2] } }
    }
    impl<T> From<(T, T, T, T)> for Extent<T, 4> {
        fn from(dim: (T, T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2, dim.3] } }
    }
}
