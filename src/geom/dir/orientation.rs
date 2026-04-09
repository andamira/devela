// devela::geom::dir::orientation
//
//! Defines [`Orientation`].
//

#[cfg(doc)]
use crate::{Distance, Position};

#[doc = crate::_tags!(geom_dir)]
/// A unitless directional vector in `D`-dimensional space.
#[doc = crate::_doc_location!("geom/dir")]
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
///
/// See also: [`Orientation1`], [`Orientation2`], [`Orientation3`].
#[must_use]
#[repr(transparent)]
pub struct Orientation<T, const D: usize> {
    /// The directional components in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom_dir)]
/// A 1-dimensional [`Orientation`].
#[doc = crate::_doc_location!("geom/dir")]
pub type Orientation1<T> = Orientation<T, 1>;

#[doc = crate::_tags!(geom_dir)]
/// A 2-dimensional [`Orientation`].
#[doc = crate::_doc_location!("geom/dir")]
pub type Orientation2<T> = Orientation<T, 2>;

#[doc = crate::_tags!(geom_dir)]
/// A 3-dimensional [`Orientation`].
#[doc = crate::_doc_location!("geom/dir")]
pub type Orientation3<T> = Orientation<T, 3>;

crate::_impl_geom_dim![common_methods: Orientation];
crate::_impl_geom_dim![common_traits: Orientation];
