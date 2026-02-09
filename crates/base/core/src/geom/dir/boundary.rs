// devela_base_core::geom::dir::boundary
//
//! Defines: [`Boundary1d`], [`Boundary2d`], [`Boundary3d`].
//!
//! Orientations relative to a bounded region along one or more axes.
//!
//! A boundary orientation identifies a side or face of a bounded region,
//! without implying motion, comparison, or violation.
//!
//! They are: discrete, region-relative, non-cyclic, without magnitude.
//!
//! In one dimension, boundary orientation corresponds to the two extrema
//! of a totally ordered extent.
//

#[cfg(doc)]
use crate::Extent;
use crate::{_impl_init, Sign};

_impl_init![ConstInitCore: Self::Left => Boundary1d, Boundary2d, Boundary3d];

#[doc = crate::_tags!(geom_dir)]
/// Orientation relative to a one-dimensional boundary.
#[doc = crate::_doc_location!("num/geom/dir")]
///
/// Identifies the lower or upper side of a bounded [`Extent<T, 1>`] or other one-dimensional
/// bounded domains, corresponding to the extrema of a totally ordered domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Boundary1d {
    /// The lower boundary side.
    Lower,
    /// The upper boundary side.
    Upper,
}
#[allow(missing_docs, non_upper_case_globals)]
impl Boundary1d {
    pub const Left: Self = Self::Lower;
    pub const Right: Self = Self::Upper;
    pub const Start: Self = Self::Lower;
    pub const End: Self = Self::Upper;
    pub const Before: Self = Self::Lower;
    pub const After: Self = Self::Upper;

    /// Returns the opposite boundary.
    pub const fn opposite(self) -> Self {
        match self {
            Self::Lower => Self::Upper,
            Self::Upper => Self::Lower,
        }
    }

    /// Converts the boundary into a numeric sign.
    ///
    /// This is meaningful only in ordered numeric contexts.
    pub const fn sign(self) -> Sign {
        match self {
            Self::Lower => Sign::Negative,
            Self::Upper => Sign::Positive,
        }
    }
}

#[doc = crate::_tags!(geom_dir)]
/// Orientation relative to a two-dimensional rectangular boundary.
#[doc = crate::_doc_location!("num/geom/dir")]
///
/// Identifies a side of a bounded [`Extent<T, 2>`], or other two-dimensional bounded domains.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Boundary2d {
    /// Left boundary face.
    Left,
    /// Right boundary face.
    Right,
    /// Bottom boundary face.
    Bottom,
    /// Top boundary face.
    Top,
}
#[allow(missing_docs, non_upper_case_globals)]
impl Boundary2d {
    pub const Above: Self = Self::Top;
    pub const Below: Self = Self::Bottom;

    /// Returns the opposite boundary.
    pub const fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Top => Self::Bottom,
        }
    }
}

#[doc = crate::_tags!(geom_dir)]
/// Orientation relative to a three-dimensional bounding volume.
#[doc = crate::_doc_location!("num/geom/dir")]
///
/// Identifies a face of a bounded [`Extent<T, 3>`], or other three-dimensional bounded domains.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Boundary3d {
    /// Left boundary face.
    Left,
    /// Right boundary face.
    Right,
    /// Bottom boundary face.
    Bottom,
    /// Top boundary face.
    Top,
    /// Front boundary face.
    Front,
    /// Back boundary face.
    Back,
}
#[allow(missing_docs, non_upper_case_globals)]
impl Boundary3d {
    pub const Above: Self = Self::Top;
    pub const Below: Self = Self::Bottom;
    pub const Near: Self = Self::Front;
    pub const Far: Self = Self::Back;

    /// Returns the opposite boundary.
    pub const fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Top => Self::Bottom,
            Self::Front => Self::Back,
            Self::Back => Self::Front,
        }
    }
}
