// devela::geom::metric::distance
//
//! Defines [`Region`], [`RegionStrided`], [`region!`].
//
// TOC
// - struct Region, type aliases
// - implementations
// - macro region!

use crate::{Extent, Position, Stride};

#[doc = crate::_tags!(geom)]
/// A [`Position`]ed [`Extent`].
#[doc = crate::_doc_location!("geom/metric")]
///
/// See also: [`Region1`], [`Region2`], [`Region3`],
/// [`RegionS`], [`RegionS1`], [`RegionS2`], [`RegionS3`].
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
#[doc = crate::_doc_location!("geom/metric")]
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

    /// Returns a new `Region` from values convertible into a position and an extent.
    pub fn from_parts<PI, EI>(pos: PI, ext: EI) -> Self
    where
        PI: Into<Position<P, D>>,
        EI: Into<Extent<E, D>>,
    {
        Self::new(pos.into(), ext.into())
    }
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

/* impls: mapping */

impl<P, E, const D: usize> Region<P, E, D> {
    #[must_use]
    /// Returns a new region by applying `f` to each position component.
    pub fn map_pos<P2>(self, f: impl FnMut(P) -> P2) -> Region<P2, E, D> {
        Region::new(self.pos.map(f), self.ext)
    }

    #[must_use]
    /// Returns a new region by applying `f` to each extent component.
    pub fn map_ext<E2>(self, f: impl FnMut(E) -> E2) -> Region<P, E2, D> {
        Region::new(self.pos, self.ext.map(f))
    }

    #[must_use]
    /// Returns a new region by applying `fpos` to the position and `fext` to the extent.
    pub fn map<P2, E2>(
        self,
        fpos: impl FnMut(P) -> P2,
        fext: impl FnMut(E) -> E2,
    ) -> Region<P2, E2, D> {
        Region::new(self.pos.map(fpos), self.ext.map(fext))
    }

    /// Returns a new region by fallibly applying `f` to each position component.
    pub fn try_map_pos<P2, X>(
        self,
        f: impl FnMut(P) -> Result<P2, X>,
    ) -> Result<Region<P2, E, D>, X> {
        Ok(Region::new(self.pos.try_map(f)?, self.ext))
    }

    /// Returns a new region by fallibly applying `f` to each extent component.
    pub fn try_map_ext<E2, X>(
        self,
        f: impl FnMut(E) -> Result<E2, X>,
    ) -> Result<Region<P, E2, D>, X> {
        Ok(Region::new(self.pos, self.ext.try_map(f)?))
    }

    /// Returns a new region by fallibly applying `fpos` to the position and
    /// `fext` to the extent.
    ///
    /// Both conversions must use the same error type.
    pub fn try_map<P2, E2, X>(
        self,
        fpos: impl FnMut(P) -> Result<P2, X>,
        fext: impl FnMut(E) -> Result<E2, X>,
    ) -> Result<Region<P2, E2, D>, X> {
        Ok(Region::new(self.pos.try_map(fpos)?, self.ext.try_map(fext)?))
    }

    #[must_use]
    /// Converts the position and extent using `From`.
    pub fn map_into<P2, E2>(self) -> Region<P2, E2, D>
    where
        P2: From<P>,
        E2: From<E>,
    {
        self.map(P2::from, E2::from)
    }

    /// Converts the position and extent using `TryFrom`.
    ///
    /// Both conversions must use the same error type.
    pub fn try_map_into<P2, E2, X>(self) -> Result<Region<P2, E2, D>, X>
    where
        P2: TryFrom<P, Error = X>,
        E2: TryFrom<E, Error = X>,
    {
        self.try_map(P2::try_from, E2::try_from)
    }
}

#[doc = crate::_tags!(geom construction)]
/// Constructs a [`Region`] from a [`Position`] and an [`Extent`].
#[doc = crate::_doc_location!("geom/metric")]
///
/// Supports:
/// - direct construction from position and extent expressions,
/// - flat or grouped construction from raw components,
/// - cast-construction for primitive position and extent scalars.
///
/// Notes:
/// - Explicit cast-construction supports 1 to 4 dimensions and is const-friendly.
/// - Cast forms delegate to [`pos!`][crate::pos] and [`ext!`][crate::ext].
/// - Whole-region cast shorthand supports any dimension and is runtime-only.
///
/// # Example
/// ```
/// # use devela::{region, pos, ext, Region2, Region3};
/// // construct
/// let a = region!(1_i32, 2, 3_u32, 4);
/// assert_eq![a, Region2::new(pos!(1, 2), ext!(3, 4))];
///
/// let b = region!((1, 2, 3), (4, 5, 6));
/// assert_eq![b, Region3::new(pos!(1, 2, 3), ext!(4, 5, 6))];
///
/// let c = region!(pos!(7, 8), ext!(9, 10));
/// assert_eq![c, Region2::new(pos!(7, 8), ext!(9, 10))];
///
/// // checked
/// let a2 = region!(checked => i16, u8; 1_i32, 2_i32; 3_u16, 4_u16);
/// assert_eq![a2, Ok(Region2::new(pos!(1_i16, 2_i16), ext!(3_u8, 4_u8)))];
///
/// // runtime shorthand over the whole region
/// let a3 = region!(wrapping a => u8, u8);
/// assert_eq![a3, Region2::new(pos!(1_u8, 2_u8), ext!(3_u8, 4_u8))];
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! region {
    (
    // flat 2D construction from raw components
     $x:expr, $y:expr, $w:expr, $h:expr $(,)?) => {
        $crate::Region::new($crate::pos!($x, $y), $crate::ext!($w, $h))
    };
    (
    // flat 3D construction from raw components
     $x:expr, $y:expr, $z:expr, $w:expr, $h:expr, $d:expr $(,)?) => {
        $crate::Region::new($crate::pos!($x, $y, $z), $crate::ext!($w, $h, $d))
    };
    (
    // grouped raw construction
     ($($pos:expr),+ $(,)?), ($($ext:expr),+ $(,)?) $(,)?
    ) => { $crate::Region::new( $crate::pos!($($pos),+), $crate::ext!($($ext),+), ) };
    (
    // direct construction from existing values
     $pos:expr, $ext:expr $(,)?
    ) => { $crate::Region::new($pos, $ext) };
    (
    // explicit component cast-construction; const-friendly
     checked => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+ $(,)?) => {
        $crate::_geom_region_cast_ctor!(checked => $P, $E; $($pos),+; $($ext),+)
    };
    (checked? => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+ $(,)?) => {
        $crate::_geom_region_cast_ctor!(checked? => $P, $E; $($pos),+; $($ext),+)
    };
    (checked_unwrap => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+ $(,)?) => {
        $crate::_geom_region_cast_ctor!(checked_unwrap => $P, $E; $($pos),+; $($ext),+)
    };
    (checked_expect => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+, $msg:expr) => {
        $crate::_geom_region_cast_ctor!(checked_expect => $P, $E; $($pos),+; $($ext),+, $msg)
    };
    (saturating => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+ $(,)?) => {
        $crate::_geom_region_cast_ctor!(saturating => $P, $E; $($pos),+; $($ext),+)
    };
    (wrapping => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+ $(,)?) => {
        $crate::_geom_region_cast_ctor!(wrapping => $P, $E; $($pos),+; $($ext),+)
    };

    (
    // whole-region cast shorthand; runtime-only
     checked $from:expr => $P:ty, $E:ty
    ) => {
        $crate::_geom_region_cast_ctor!(checked $from => $P, $E)
    };
    (checked? $from:expr => $P:ty, $E:ty) => {
        $crate::_geom_region_cast_ctor!(checked? $from => $P, $E)
    };
    (checked_unwrap $from:expr => $P:ty, $E:ty) => {
        $crate::_geom_region_cast_ctor!(checked_unwrap $from => $P, $E)
    };
    (checked_expect $from:expr => $P:ty, $E:ty, $msg:expr) => {
        $crate::_geom_region_cast_ctor!(checked_expect $from => $P, $E, $msg)
    };
    (saturating $from:expr => $P:ty, $E:ty) => {
        $crate::_geom_region_cast_ctor!(saturating $from => $P, $E)
    };
    (wrapping $from:expr => $P:ty, $E:ty) => {
        $crate::_geom_region_cast_ctor!(wrapping $from => $P, $E)
    };
}
#[doc(inline)]
pub use region;

#[cfg(test)]
mod tests {
    use crate::{Region2, Region3, ext, pos};

    #[test]
    fn test_region_construction() {
        // flat from raw components
        let rf2 = region!(2, 5, 7, 9);
        assert_eq![rf2, Region2::new(pos!(2, 5), ext!(7, 9))];
        let rf3 = region!(2, 5, 3, 7, 9, 4);
        assert_eq![rf3, Region3::new(pos!(2, 5, 3), ext!(7, 9, 4),)];

        // grouped
        let r2 = region!((2, 5), (7, 9));
        assert_eq![r2, Region2::new(pos!(2, 5), ext!(7, 9))];

        // direct
        let r1 = region!(pos!(2, 5), ext!(7, 9));
        assert_eq![r1, Region2::new(pos!(2, 5), ext!(7, 9))];
    }
    #[test]
    fn test_region_checked_casts() {
        let r = region!((2_i32, 5_i32), (7_u16, 9_u16));

        let a = region!(checked => u8, u8; 2_i32, 5_i32; 7_u16, 9_u16);
        assert_eq![a, Ok(Region2::new(pos!(2_u8, 5_u8), ext!(7_u8, 9_u8)))];

        let b = region!(checked r => u8, u8);
        assert_eq![b, Ok(Region2::new(pos!(2_u8, 5_u8), ext!(7_u8, 9_u8)))];
    }
    #[test]
    fn test_region_plain_casts() {
        let r = region!((300_i32, -5_i32), (260_i32, -1_i32));

        let s = region!(saturating r => u8, u8);
        assert_eq![s, Region2::new(pos!(255_u8, 0_u8), ext!(255_u8, 0_u8))];

        let w = region!(wrapping r => u8, u8);
        assert_eq![w, Region2::new(pos!(44_u8, 251_u8), ext!(4_u8, 255_u8))];
    }
}
