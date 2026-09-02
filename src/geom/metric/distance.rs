// devela/src/geom/metric/distance.rs
//
//! Defines [`Distance`][1|2|3].
//

#[cfg(doc)]
use crate::{Extent, Orientation, Position};

#[doc = crate::_tags!(geom)]
/// A component-wise separation between locations in `D`-dimensional space.
#[doc = crate::_doc_meta!{
    location("geom/metric", struct Distance),
    test_size_of(Distance<f32, 2> = 8|64; niche !Option),
}]
/// `Distance` describes how far apart two locations are along each coordinate
/// axis, without assigning a signed direction to that separation.
///
/// Its components are conventionally non-negative,
/// although this type does not enforce that invariant.
///
/// - Unlike [`Position`], `Distance` is relative rather than absolute.
/// - Unlike [`Orientation`], `Distance` has **magnitude** but no defined direction.
/// - Unlike [`Extent`], `Distance` relates locations rather than describing
///   the size of an object or domain.
/// - Unlike [`Vector`], `Distance` does not represent a directed displacement.
///
/// See also: [`Distance1`], [`Distance2`], [`Distance3`], [`dis!`][crate::dis].
///
/// [`Vector`]: crate::Vector
#[must_use]
#[repr(transparent)]
pub struct Distance<T, const D: usize> {
    /// The component-wise separation in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Distance`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Distance1),
    test_size_of(Distance1<f32> = 4|32; niche !Option),
}]
pub type Distance1<T> = Distance<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Distance`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Distance3),
    test_size_of(Distance2<f32> = 8|64; niche !Option),
}]
pub type Distance2<T> = Distance<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Distance`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Distance3),
    test_size_of(Distance3<f32> = 12|96; niche !Option),
}]
pub type Distance3<T> = Distance<T, 3>;

crate::_geom_dim_impl_common![common_methods: Distance];
crate::_geom_dim_impl_common![common_traits: Distance];
