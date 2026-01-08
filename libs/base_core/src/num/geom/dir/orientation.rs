// devela_base_core::num::geom::metric::direction
//
//! Defines [`Orientation`].
//

#[cfg(doc)]
use crate::{Distance, Position};

#[doc = crate::_tags!(geom)]
/// A unitless directional vector in `D`-dimensional space.
#[doc = crate::_doc_location!("num/geom/dir")]
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

crate::_impl_geom_dim![common_methods: Orientation];
crate::_impl_geom_dim![common_traits: Orientation];
