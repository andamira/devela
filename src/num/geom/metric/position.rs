// devela::num::geom::metric::position
//
//! Defines [`Position`].
//

#[cfg(doc)]
use crate::{Distance, Orientation};

#[doc = crate::TAG_GEOM!()]
/// A location in `D`-dimensional space.
///
/// Represents an absolute position in a coordinate system.
///
/// - Unlike [`Distance`], `Position` is **not relative**, it describes
///   a fixed location rather than a displacement.
/// - Unlike [`Orientation`], `Position` has **magnitude and reference**,
///   but no inherent orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position<T, const D: usize> {
    /// The coordinate values in `D`-dimensional space.
    pub dim: [T; D],
}
