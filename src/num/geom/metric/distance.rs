// devela::num::geom::metric::distance
//
//! Defines [`Distance`].
//

#[cfg(doc)]
use crate::{Orientation, Position};

#[doc = crate::TAG_GEOM!()]
/// A separation between two locations in `D`-dimensional space.
///
/// Represents a displacement vector **without an absolute origin**.
/// It describes the magnitude of separation between positions.
///
/// - Unlike [`Position`], `Distance` is **relative**,
///   and represents how far apart two positions are.
/// - Unlike [`Orientation`], `Distance` has **magnitude**
///   but no defined **orientation**.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distance<T, const D: usize> {
    /// The component-wise separation in `D`-dimensional space.
    pub dim: [T; D],
}
