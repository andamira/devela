// devela_base_core::geom::metric::distance
//
//! Defines [`Region`], [`RegionStrided`].
//

use crate::{Extent, Position, Stride};

#[doc = crate::_tags!(geom)]
/// A [`Position`]ed [`Extent`].
///
/// See also: [`Region1`], [`Region2`], [`Region3`],
/// [`RegionS`], [`RegionS1`], [`RegionS2`], [`RegionS3`].
#[doc = crate::_doc_location!("geom/metric")]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Region<P, E, const D: usize> {
    ///
    pub pos: Position<P, D>,
    ///
    pub ext: Extent<E, D>,
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Region`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Region1<P, E> = Region<P, E, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Region`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Region2<P, E> = Region<P, E, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Region`].
#[doc = crate::_doc_location!("geom/metric")]
pub type Region3<P, E> = Region<P, E, 3>;

#[doc = crate::_tags!(geom)]
/// A [`Position`]ed [`Extent`] sharing the **S**ame type.
#[doc = crate::_doc_location!("geom/metric")]
pub type RegionS<T, const D: usize> = Region<T, T, D>;

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`RegionS`].
#[doc = crate::_doc_location!("geom/metric")]
pub type RegionS1<T> = RegionS<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`RegionS`].
#[doc = crate::_doc_location!("geom/metric")]
pub type RegionS2<T> = RegionS<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`RegionS`].
#[doc = crate::_doc_location!("geom/metric")]
pub type RegionS3<T> = RegionS<T, 3>;

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
pub struct RegionStrided<P, E, const D: usize> {
    /// The positioned extent.
    pub region: Region<P, E, D>,
    /// The step size per dimension.
    pub stride: Stride<E, D>,
}

/* impls: constructors */

#[rustfmt::skip]
impl<P, E, const D: usize> Region<P, E, D> {
    /// Returns a new `Region` from a `pos`ition and an `ext`ent.
    pub const fn new(pos: Position<P, D>, ext: Extent<E, D>) -> Self { Self { pos, ext } }
}
impl<P, E> From<((P, P), (E, E))> for Region<P, E, 2> {
    fn from(((x, y), (w, h)): ((P, P), (E, E))) -> Self {
        Self {
            pos: Position::from((x, y)),
            ext: Extent::from((w, h)),
        }
    }
}
impl<P, E> From<((P, P, P), (E, E, E))> for Region<P, E, 3> {
    fn from(((x, y, z), (w, h, d)): ((P, P, P), (E, E, E))) -> Self {
        Self {
            pos: Position::from((x, y, z)),
            ext: Extent::from((w, h, d)),
        }
    }
}
impl<P, E, const D: usize> From<(Position<P, D>, Extent<E, D>)> for Region<P, E, D> {
    fn from(from: (Position<P, D>, Extent<E, D>)) -> Self {
        Self::new(from.0, from.1)
    }
}
impl<P, E, const D: usize> From<(Extent<E, D>, Position<P, D>)> for Region<P, E, D> {
    fn from(from: (Extent<E, D>, Position<P, D>)) -> Self {
        Self::new(from.1, from.0)
    }
}

/* impls: accessors */

#[rustfmt::skip]
impl<P: Copy, E: Copy> Region2<P, E> {
    #[must_use]
    /// Returns a copy of the first position dimension `x`.
    pub const fn x(self) -> P { self.pos.x() }
    #[must_use]
    /// Returns a copy of the second position dimension `y`.
    pub const fn y(self) -> P { self.pos.y() }

    #[must_use]
    /// Returns a copy of the first extent dimension `w`idth.
    pub const fn w(self) -> E { self.ext.w() }
    #[must_use]
    /// Returns a copy of the second extent dimension `h`eight.
    pub const fn h(self) -> E { self.ext.h() }
}

#[rustfmt::skip]
impl<P: Copy, E: Copy> Region3<P, E> {
    #[must_use]
    /// Returns a copy of the first position dimension `x`.
    pub const fn x(self) -> P { self.pos.x() }
    #[must_use]
    /// Returns a copy of the second position dimension `y`.
    pub const fn y(self) -> P { self.pos.y() }
    #[must_use]
    /// Returns a copy of the third position dimension `z`.
    pub const fn z(self) -> P { self.pos.z() }

    #[must_use]
    /// Returns a copy of the first extent dimension `w`idth.
    pub const fn w(self) -> E { self.ext.w() }
    #[must_use]
    /// Returns a copy of the second extent dimension `h`eight.
    pub const fn h(self) -> E { self.ext.h() }
    #[must_use]
    /// Returns a copy of the third extent dimension `d`epth.
    pub const fn d(self) -> E { self.ext.d() }
}
