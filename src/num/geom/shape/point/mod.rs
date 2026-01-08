// devela::num::geom::shape::point
//
//! A geometrical point.
//

#[cfg(feature = "alloc")]
use crate::Vec;

mod impl_traits;
mod methods;

#[doc = crate::_tags!(geom)]
/// A coordinate position in `D`-space without extent.
#[doc = crate::_doc_location!("num/geom/shape")]
#[must_use]
pub struct Point<T, const D: usize> {
    /// The D-dimensional coordinates.
    pub coords: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A specific position in 2d-space without a size.
#[doc = crate::_doc_location!("num/geom/shape")]
pub type Point2d<T> = Point<T, 2>;

#[doc = crate::_tags!(geom)]
/// A specific position in 3d-space without a size.
#[doc = crate::_doc_location!("num/geom/shape")]
pub type Point3d<T> = Point<T, 3>;

/* lists */

#[doc = crate::_tags!(geom)]
/// A static sequence of `N` `D`-dimensional [`Point`]s.
#[doc = crate::_doc_location!("num/geom/shape")]
#[must_use]
#[repr(transparent)]
#[derive(Debug)]
pub struct Points<T, const D: usize, const N: usize> {
    /// The array of points.
    pub array: [Point<T, D>; N],
}
#[doc = crate::_tags!(geom)]
/// A static sequence of 2-dimensional [`Point`]s.
#[doc = crate::_doc_location!("num/geom/shape")]
pub type Points2d<T, const N: usize> = Points<T, 2, N>;

#[doc = crate::_tags!(geom)]
/// A dynamic sequence of `D`-dimensional [`Point`]s.
#[doc = crate::_doc_location!("num/geom/shape")]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[derive(Debug)]
pub struct VecPoints<T, const D: usize> {
    /// The vec of points.
    pub vec: Vec<Point<T, D>>,
}
