// devela::num::geom::shape::point
//
//! A geometrical point.
//

#[cfg(feature = "alloc")]
use crate::data::Vec;

mod impl_traits;
mod methods;

/// A coordinate position in `D`-space without extent.
pub struct Point<T, const D: usize> {
    /// The D-dimensional coordinates.
    pub coords: [T; D],
}

/// A specific position in 2d-space without a size.
pub type Point2d<T> = Point<T, 2>;

/// A specific position in 3d-space without a size.
pub type Point3d<T> = Point<T, 3>;

/// A static sequence of `N` `D`-dimensional [`Point`]s.
#[must_use]
#[repr(transparent)]
pub struct Points<T, const D: usize, const N: usize> {
    /// The array of points.
    pub array: [Point<T, D>; N],
}

/// A dynamic sequence of `D`-dimensional [`Point`]s.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub struct VecPoints<T, const D: usize> {
    /// The vec of points.
    pub vec: Vec<Point<T, D>>,
}
