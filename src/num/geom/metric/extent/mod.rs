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
    /// The size values in `D`-dimensional space.
    pub dim: [T; D],
}
