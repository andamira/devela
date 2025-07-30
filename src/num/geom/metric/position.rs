// devela::num::geom::metric::position
//
//! Defines [`Position`].
//

#[cfg(doc)]
use crate::{Distance, Orientation};

#[doc = crate::TAG_GEOM!()]
/// A location in `D`-dimensional space.
///
/// Represents an absolute position in a coordinate system.
///
/// - Unlike [`Distance`], `Position` is **not relative**, it describes
///   a fixed location rather than a displacement.
/// - Unlike [`Orientation`], `Position` has **magnitude and reference**,
///   but no inherent orientation.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position<T, const D: usize> {
    /// The coordinate values in `D`-dimensional space.
    pub dim: [T; D],
}
crate::items! {
    impl<T, const D: usize> From<[T; D]> for Position<T, D> {
        fn from(dim: [T; D]) -> Self { Self { dim } }
    }
    impl<T> From<(T, T)> for Position<T, 2> {
        fn from(dim: (T, T)) -> Self { Self { dim: [dim.0, dim.1] } }
    }
    impl<T> From<(T, T, T)> for Position<T, 3> {
        fn from(dim: (T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2] } }
    }
    impl<T> From<(T, T, T, T)> for Position<T, 4> {
        fn from(dim: (T, T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2, dim.3] } }
    }
}
