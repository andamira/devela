// devela::geom::metric::macros
//
//! Defines [`dis!`], [`ext!`], [`pos`], [`region!`].
//
// TOC
// - define dis!, ext!, pos!
// - define region!
// - macro tests

#[cfg(doc)]
use crate::{Distance, Extent, Position, Region};

crate::_define_geom_dim_macro![($) dis, "a", Distance, geom, "geom/metric"];
crate::_define_geom_dim_macro![($) ext, "an", Extent, geom, "geom/metric"];
crate::_define_geom_dim_macro![($) pos, "a", Position, geom, "geom/metric"];

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
    use crate::{
        Distance, Distance2, Extent, Extent2, Extent3, Position, Position2, Region2, Region3, dis,
        ext, pos,
    };

    #[test]
    fn dis_macro_surface() {
        assert_eq![Distance::<i32, 1>::new([2]), dis!(2)];
        assert_eq![Distance2::<i32>::new([2, 5]), dis!(2, 5)];
        assert_eq![dis!([3; 6]).dim, [3, 3, 3, 3, 3, 3]];

        let p = dis!(2_i32, 5_i32);
        assert_eq![dis!(checked p => u8), Ok(Distance2::<u8>::new([2, 5]))];
        assert_eq![dis!(saturating => u8; 300_i32, -5_i32), Distance2::<u8>::new([255, 0])];
    }
    #[test]
    fn ext_macro_constructors() {
        let e = ext!([3; 8]);
        assert_eq![e.dim, [3, 3, 3, 3, 3, 3, 3, 3]];

        assert_eq![Extent::<i32, 1>::new([2]), ext!(2)];
        assert_eq![Extent2::<i32>::new([2, 5]), ext!(2, 5)];
        assert_eq![Extent3::<i32>::new([2, 5, 6]), ext!(2, 5, 6)];
        assert_eq![Extent::<i32, 4>::new([2, 5, 6, 7]), ext!(2, 5, 6, 7)];
    }
    #[test]
    fn ext_macro_checked_casts() {
        let a = ext!(2_i32, 5);

        let b = ext!(checked => u8; a.x(), a.y());
        assert_eq![b, Ok(Extent2::new([2_u8, 5]))];

        let c = ext!(checked a => u8);
        assert_eq![c, Ok(Extent2::new([2_u8, 5]))];
    }
    #[test]
    fn ext_macro_saturating_and_wrapping() {
        let a = ext!(300_i32, -5_i32);

        let s = ext!(saturating => u8; a.x(), a.y());
        assert_eq![s, Extent2::new([255_u8, 0])];

        let w = ext!(wrapping => u8; a.x(), a.y());
        assert_eq![w, Extent2::new([44_u8, 251])];

        let s2 = ext!(saturating a => u8);
        assert_eq![s2, Extent2::new([255_u8, 0])];

        let w2 = ext!(wrapping a => u8);
        assert_eq![w2, Extent2::new([44_u8, 251])];
    }
    #[test]
    fn pos_macro_surface() {
        assert_eq![Position::<i32, 1>::new([2]), pos!(2)];
        assert_eq![Position2::<i32>::new([2, 5]), pos!(2, 5)];
        assert_eq![pos!([3; 6]).dim, [3, 3, 3, 3, 3, 3]];

        let p = pos!(2_i32, 5_i32);
        assert_eq![pos!(checked p => u8), Ok(Position2::<u8>::new([2, 5]))];
        assert_eq![pos!(saturating => u8; 300_i32, -5_i32), Position2::<u8>::new([255, 0])];
    }
    #[test]
    fn region_macro_construction() {
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
    fn region_macro_checked_casts() {
        let r = region!((2_i32, 5_i32), (7_u16, 9_u16));

        let a = region!(checked => u8, u8; 2_i32, 5_i32; 7_u16, 9_u16);
        assert_eq![a, Ok(Region2::new(pos!(2_u8, 5_u8), ext!(7_u8, 9_u8)))];

        let b = region!(checked r => u8, u8);
        assert_eq![b, Ok(Region2::new(pos!(2_u8, 5_u8), ext!(7_u8, 9_u8)))];
    }
    #[test]
    fn region_macro_plain_casts() {
        let r = region!((300_i32, -5_i32), (260_i32, -1_i32));

        let s = region!(saturating r => u8, u8);
        assert_eq![s, Region2::new(pos!(255_u8, 0_u8), ext!(255_u8, 0_u8))];

        let w = region!(wrapping r => u8, u8);
        assert_eq![w, Region2::new(pos!(44_u8, 251_u8), ext!(4_u8, 255_u8))];
    }
}
