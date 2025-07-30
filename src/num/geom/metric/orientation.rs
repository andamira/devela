// devela::num::geom::metric::direction
//
//! Defines [`Orientation`].
//

#[cfg(doc)]
use crate::{Distance, Position};

#[doc = crate::TAG_GEOM!()]
/// A unitless directional vector in `D`-dimensional space.
///
/// Represents **only the direction of movement**, without an absolute
/// reference point or inherent magnitude. It is **typically normalized**
/// to remove scale dependence.
///
/// - Unlike [`Position`], `Orientation` **does not describe a fixed location**.
/// - Unlike [`Distance`], `Orientation` **does not measure separation**.
///
/// This type does **not enforce normalization**, but it is expected
/// to be normalized in most use cases.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Orientation<T, const D: usize> {
    /// The directional components in `D`-dimensional space.
    pub dim: [T; D],
}
crate::items! {
    impl<T, const D: usize> From<[T; D]> for Orientation<T, D> {
        fn from(dim: [T; D]) -> Self { Self { dim } }
    }
    impl<T> From<(T, T)> for Orientation<T, 2> {
        fn from(dim: (T, T)) -> Self { Self { dim: [dim.0, dim.1] } }
    }
    impl<T> From<(T, T, T)> for Orientation<T, 3> {
        fn from(dim: (T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2] } }
    }
    impl<T> From<(T, T, T, T)> for Orientation<T, 4> {
        fn from(dim: (T, T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2, dim.3] } }
    }
}
