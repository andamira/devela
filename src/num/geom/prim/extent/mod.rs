// devela::num::geom::prim::extent
//
//! A geometrical extent.
//

mod core_traits;
mod methods;

/// An orthogonal extension in `D`-space without a coordinate position.
///
/// It has the implied shape of an orthotope (a generalized rectangle).
#[must_use]
#[repr(transparent)]
pub struct Extent<T, const D: usize> {
    /// The D-dimensional extent.
    extent: [T; D],
}

/// A 2-dimensional [`Extent`].
pub type Extent2d<T> = Extent<T, 2>;

/// A 3-dimensional [`Extent`].
pub type Extent3d<T> = Extent<T, 3>;
