// devela::num::geom::metric::distance
//
//! Defines [`Distance`].
//

#[cfg(all(doc, feature = "metric"))]
use crate::Orientation;
#[cfg(doc)]
use crate::Position;

#[doc = crate::_TAG_GEOM!()]
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
pub struct Distance<T, const D: usize> {
    /// The component-wise separation in `D`-dimensional space.
    pub dim: [T; D],
}

crate::_impl_metric![common_methods: Distance];
crate::_impl_metric![common_traits: Distance];
