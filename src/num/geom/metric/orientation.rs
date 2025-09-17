// devela::num::geom::metric::direction
//
//! Defines [`Orientation`].
//

#[cfg(doc)]
use crate::{Distance, Position};

#[doc = crate::_TAG_GEOM!()]
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
pub struct Orientation<T, const D: usize> {
    /// The directional components in `D`-dimensional space.
    pub dim: [T; D],
}

crate::_impl_metric![common_methods: Orientation];
crate::_impl_metric![common_traits: Orientation];
