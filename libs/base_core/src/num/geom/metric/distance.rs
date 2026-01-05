// devela_base_core::num::geom::metric::distance
//
//! Defines [`Distance`].
//

#[cfg(doc)]
use crate::{Orientation, Position};

#[doc = crate::_TAG_GEOM!()]
/// A separation between two locations in `D`-dimensional space.
#[doc = crate::_doc_location!("num/geom/metric")]
///
/// Represents a displacement vector **without an absolute origin**.
/// It describes the magnitude of separation between positions.
///
/// - Unlike [`Position`], `Distance` is **relative**,
///   and represents how far apart two positions are.
/// - Unlike [`Orientation`], `Distance` has **magnitude** but no defined direction.
#[must_use]
#[repr(transparent)]
pub struct Distance<T, const D: usize> {
    /// The component-wise separation in `D`-dimensional space.
    pub dim: [T; D],
}

crate::_impl_geom_dim![common_methods: Distance];
crate::_impl_geom_dim![common_traits: Distance];
