// devela::geom::metric::position
//
//! Defines [`Position`].
//

#[cfg(doc)]
use crate::{Distance, Orientation};

#[doc = crate::_tags!(geom)]
/// A location in `D`-dimensional space.
#[doc = crate::_doc_location!("geom/metric")]
///
/// Represents an absolute position in a coordinate system.
///
/// - Unlike [`Distance`], `Position` is **not relative**, it describes
///   a fixed location rather than a displacement.
/// - Unlike [`Orientation`], `Position` is a **location**, not a direction.
///
/// See also: [`Position1`], [`Position2`], [`Position3`].
#[must_use]
#[repr(transparent)]
pub struct Position<T, const D: usize> {
    /// The coordinate values in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Position`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Position1<T> = Position<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Position`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Position2<T> = Position<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Position`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Position3<T> = Position<T, 3>;

crate::_impl_geom_dim![common_methods: Position];
crate::_impl_geom_dim![common_traits: Position];
