// devela/src/geom/metric/position.rs
//
//! Defines [`Position`][1|2|3].
//!
//! > A geometrical position.
//

#[cfg(doc)]
use crate::{Distance, Orientation};

#[doc = crate::_tags!(geom)]
/// A location in `D`-dimensional space.
#[doc = crate::_doc_meta!{
    location("geom/metric", struct Position),
    test_size_of(Position<f32, 2> = 8|64; niche !Option),
}]
/// Represents an absolute position in a coordinate system.
///
/// - Unlike [`Distance`], `Position` is **not relative**, it describes
///   a fixed location rather than a displacement.
/// - Unlike [`Orientation`], `Position` is a **location**, not a direction.
///
/// See also: [`Position1`], [`Position2`], [`Position3`], [`pos!`][crate::pos].
#[must_use]
#[repr(transparent)]
pub struct Position<T, const D: usize> {
    /// The coordinate values in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Position`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Position1),
    test_size_of(Position1<f32> = 4|32; niche !Option),
}]
pub type Position1<T> = Position<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Position`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Position2),
    test_size_of(Position2<f32> = 8|64; niche !Option),
}]
pub type Position2<T> = Position<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Position`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Position3),
    test_size_of(Position3<f32> = 12|96; niche !Option),
}]
pub type Position3<T> = Position<T, 3>;

crate::_geom_dim_impl_common![common_methods: Position];
crate::_geom_dim_impl_common![common_traits: Position];
