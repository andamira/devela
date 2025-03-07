// devela::num::geom::metric::extent
//
//! A geometrical extent.
//

mod impl_traits;
mod methods;

#[doc = crate::TAG_GEOM!()]
/// An orthogonal extension in `D`-space without a coordinate position.
///
/// Represents the lengths of each dimension in a multi-dimensional space,
/// providing an origin-agnostic shape with the implied form of an orthotope
/// (generalized rectangle or box).
#[must_use]
#[repr(transparent)]
pub struct Extent<T, const D: usize> {
    /// The D-dimensional size.
    pub size: [T; D],
}

#[doc = crate::TAG_GEOM!()]
/// A 2-dimensional [`Extent`].
pub type Extent2d<T> = Extent<T, 2>;

#[doc = crate::TAG_GEOM!()]
/// A 3-dimensional [`Extent`].
pub type Extent3d<T> = Extent<T, 3>;
