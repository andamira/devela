// devela::num::geom::metric::distance
//
//! Defines [`Distance`].
//

#[cfg(doc)]
use crate::{Orientation, Position};

#[doc = crate::TAG_GEOM!()]
/// A separation between two locations in `D`-dimensional space.
///
/// Represents a displacement vector **without an absolute origin**.
/// It describes the magnitude of separation between positions.
///
/// - Unlike [`Position`], `Distance` is **relative**,
///   and represents how far apart two positions are.
/// - Unlike [`Orientation`], `Distance` has **magnitude**
///   but no defined **orientation**.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distance<T, const D: usize> {
    /// The component-wise separation in `D`-dimensional space.
    pub dim: [T; D],
}
crate::items! {
    impl<T, const D: usize> From<[T; D]> for Distance<T, D> {
        fn from(dim: [T; D]) -> Self { Self { dim } }
    }
    impl<T> From<(T, T)> for Distance<T, 2> {
        fn from(dim: (T, T)) -> Self { Self { dim: [dim.0, dim.1] } }
    }
    impl<T> From<(T, T, T)> for Distance<T, 3> {
        fn from(dim: (T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2] } }
    }
    impl<T> From<(T, T, T, T)> for Distance<T, 4> {
        fn from(dim: (T, T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2, dim.3] } }
    }
}
