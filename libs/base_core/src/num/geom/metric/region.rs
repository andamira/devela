// devela_base_core::num::geom::metric::distance
//
//! Defines [`Region`], [`RegionStrided`].
//

use crate::{Extent, Position, Stride};

#[doc = crate::_tags!(geom)]
/// A [`Position`]ed [`Extent`].
#[doc = crate::_doc_location!("num/geom/metric")]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Region<T, const D: usize> {
    ///
    pub pos: Position<T, D>,
    ///
    pub size: Extent<T, D>,
}

#[doc = crate::_tags!(geom)]
/// A [`Stride`]d [`Region`] defining structured traversal.
///
/// `RegionStrided` extends `Region` by adding a stride, allowing
/// structured access to subregions or non-contiguous patterns.
///
/// - Used in **grids, datasets, and memory layouts**.
/// - Supports **efficient structured stepping** (e.g. row-major iteration).
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionStrided<T, const D: usize> {
    /// The positioned extent.
    pub region: Region<T, D>,
    /// The step size per dimension.
    pub stride: Stride<T, D>,
}

/* impls */

#[rustfmt::skip]
impl<T, const D: usize> Region<T, D> {
    /// Returns a new `Region` from a `pos`ition and a `size`.
    pub const fn new(pos: Position<T, D>, size: Extent<T, D>) -> Self { Self { pos, size } }

    ///
    pub fn position(&self) -> Position<T, D> where T: Clone { self.pos.clone() }
    ///
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
