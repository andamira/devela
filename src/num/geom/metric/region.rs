// devela::num::geom::metric::distance
//
//! Defines [`Region`], [`RegionStrided`].
//

use crate::{Extent, Position, Stride};

#[doc = crate::TAG_GEOM!()]
/// A [`Position`]ed [`Extent`].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Region<T, const D: usize> {
    ///
    pub pos: Position<T, D>,
    ///
    pub size: Extent<T, D>,
}

#[doc = crate::TAG_GEOM!()]
/// A 2-dimensional [`Region`].
pub type Region2d<T> = Region<T, 2>;
#[doc = crate::TAG_GEOM!()]
/// A 3-dimensional [`Region`].
pub type Region3d<T> = Region<T, 3>;

#[doc = crate::TAG_GEOM!()]
/// A [`Stride`]d [`Region`] defining structured traversal.
///
/// `RegionStrided` extends `Region` by adding a stride, allowing
/// structured access to subregions or non-contiguous patterns.
///
/// - Used in **grids, datasets, and memory layouts**.
/// - Supports **efficient structured stepping** (e.g. row-major iteration).
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionStrided<T, const D: usize> {
    /// The positioned extent.
    pub region: Region<T, D>,
    /// The step size per dimension.
    pub stride: Stride<T, D>,
}

/* impls */

impl<T, const D: usize> Region<T, D> {
    /// Returns a new `Region` from a `pos`ition and a `size`.
    pub const fn new(pos: Position<T, D>, size: Extent<T, D>) -> Self {
        Self { pos, size }
    }

    ///
    #[must_use] #[rustfmt::skip]
    pub fn position(&self) -> Position<T, D> where T: Clone { self.pos.clone() }
    ///
    #[rustfmt::skip]
    pub fn extent(&self) -> Extent<T, D> where T: Clone { self.size.clone() }
}
impl<T, const D: usize> From<(Position<T, D>, Extent<T, D>)> for Region<T, D> {
    fn from(from: (Position<T, D>, Extent<T, D>)) -> Self {
        Self::new(from.0, from.1)
    }
}
impl<T, const D: usize> From<(Extent<T, D>, Position<T, D>)> for Region<T, D> {
    fn from(from: (Extent<T, D>, Position<T, D>)) -> Self {
        Self::new(from.1, from.0)
    }
}
