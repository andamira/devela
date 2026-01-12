// devela_base_core::geom::metric::position
//
//! Defines [`Position`].
//

#[cfg(doc)]
use crate::{Distance, Orientation};

#[doc = crate::_tags!(geom)]
/// A location in `D`-dimensional space.
#[doc = crate::_doc_location!("num/geom/metric")]
///
/// Represents an absolute position in a coordinate system.
///
/// - Unlike [`Distance`], `Position` is **not relative**, it describes
///   a fixed location rather than a displacement.
/// - Unlike [`Orientation`], `Position` is a **location**, not a direction.
#[must_use]
#[repr(transparent)]
pub struct Position<T, const D: usize> {
    /// The coordinate values in `D`-dimensional space.
    pub dim: [T; D],
}

crate::_impl_geom_dim![common_methods: Position];
crate::_impl_geom_dim![common_traits: Position];
