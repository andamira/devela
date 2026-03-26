// devela_base_core::geom::metric::distance
//
//! Defines [`Distance`].
//

#[cfg(doc)]
use crate::{Orientation, Position};

#[doc = crate::_tags!(geom)]
/// A separation between two locations in `D`-dimensional space.
#[doc = crate::_doc_location!("geom/metric")]
///
/// Represents a displacement vector **without an absolute origin**.
/// It describes the magnitude of separation between positions.
///
/// - Unlike [`Position`], `Distance` is **relative**,
///   and represents how far apart two positions are.
/// - Unlike [`Orientation`], `Distance` has **magnitude** but no defined direction.
///
/// See also: [`Distance1`], [`Distance2`], [`Distance3`].
#[must_use]
#[repr(transparent)]
pub struct Distance<T, const D: usize> {
    /// The component-wise separation in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Distance`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Distance1<T> = Distance<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Distance`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Distance2<T> = Distance<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Distance`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Distance3<T> = Distance<T, 3>;

crate::_impl_geom_dim![common_methods: Distance];
crate::_impl_geom_dim![common_traits: Distance];
