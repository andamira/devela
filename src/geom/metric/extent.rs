// devela::geom::metric::extent
//
//! Defines [`Extent`][1|2|3], `ext!`.
//!
//! > A geometrical extent.
//
// TOC
// - struct Extent, type aliases, macro ext!
// - implementations
//
// IMPROVE: use TBD NumConst::ONE and unify methods for int and floats.

use crate::{is, whilst};

#[doc = crate::_tags!(geom)]
/// An orthogonal extension in `D`-space without a coordinate position.
#[doc = crate::_doc_location!("geom/metric")]
///
/// Represents the lengths of each dimension in a multi-dimensional space,
/// providing an origin-agnostic shape with the implied form of an orthotope
/// (generalized rectangle or box).
///
/// See also [`Extent1`], [`Extent2`], [`Extent3`], [`ext!`].
#[must_use]
#[repr(transparent)]
#[doc(alias = "Size")]
pub struct Extent<T, const D: usize> {
    /// The size values in `D`-dimensional space.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Extent`].
#[doc = crate::_doc_location!("geom/metric")]
#[doc(alias = "Size")]
pub type Extent1<T> = Extent<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Extent`].
#[doc = crate::_doc_location!("geom/metric")]
#[doc(alias = "Size")]
pub type Extent2<T> = Extent<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Extent`].
#[doc = crate::_doc_location!("geom/metric")]
#[doc(alias = "Size")]
pub type Extent3<T> = Extent<T, 3>;

crate::_define_geom_dim_macro![($) ext, "an", Extent, geom, "geom/metric"];

crate::_impl_geom_dim![common_methods: Extent];
crate::_impl_geom_dim![common_traits: Extent];

/// Implement `Extent` methods for all primitives.
macro_rules! impl_extent {
    () => {
        impl_extent![sint i8, i16, i32, i64, i128, isize];
        impl_extent![uint u8, u16, u32, u64, u128, usize];
        impl_extent![float f32, f64];
    };
    // integers common methods
    //
    // $t: the inner integer primitive type
    (int $($t:ty),+) => { $( impl_extent![@int $t]; )+ };
    (@int $t:ty) => {
        impl<const D: usize> Extent<$t, D> {
            #[must_use]
            /// Returns the internal measure, the product of the extents.
            ///
            /// It's equivalent to length, area, and volume in 1, 2 and 3 dimensions.
            pub const fn measure(self) -> $t {
                let mut measure = 1;
                whilst!(i in 0..D; measure *= self.dim[i]);
                measure
            }
            #[must_use]
            /// Returns the external boundary, the sum of the extents.
            ///
            /// It's equivalent to 2, perimeter and surface area in 1, 2 and 3 dimensions.
            pub const fn boundary(self) -> $t {
                let mut boundary = 0;
                whilst!(i in 0..D; {
                    let mut face_measure = 1;
                    whilst!(j in 0..D; is![i != j, face_measure *= self.dim[j]]);
                    boundary += face_measure;
                });
                2 * boundary // Each dimension's contribution is counted twice
            }
        }

        // impl Extent<$t, 1> {}
        impl Extent2<$t> {
            #[must_use]
            /// The area of the 2d extent.
            pub const fn area(self) -> $t { self.dim[0] * self.dim[1] }
            #[must_use]
            /// The perimeter of the 2d extent.
            pub const fn perimeter(self) -> $t { 2 * (self.dim[0] + self.dim[1]) }
        }
        impl Extent3<$t> {
            #[must_use]
            /// The volume of the 3d extent.
            pub const fn volume(self) -> $t {
                self.dim[0] * self.dim[1] * self.dim[2]
            }
            #[must_use]
            /// The surface area of the 3d extent.
            pub const fn surface_area(self) -> $t {
                2 * (self.dim[0] * self.dim[1]
                    + self.dim[1] * self.dim[2]
                    + self.dim[2] * self.dim[0])
            }
        }
    };

    (sint $($t:ty),+) => { $( impl_extent![@sint $t]; )+ };
    (@sint $t:ty ) => {
        impl_extent![int $t];
    };
    (uint $($t:ty),+) => { $( impl_extent![@uint $t]; )+ };
    (@uint $t:ty ) => {
        impl_extent![int $t];
    };

    // $f: the inner floating-point primitive type
    (float $($f:ty),+) => { $( impl_extent![@float $f]; )+ };
    (@float $f:ty) => {
        impl<const D: usize> Extent<$f, D> {
            #[must_use]
            /// Returns the internal measure, the product of the extents.
            ///
            /// It's equivalent to length, area, and volume in 1, 2 and 3 dimensions.
            pub const fn measure(self) -> $f {
                let mut measure = 1.0;
                whilst!(i in 0..D; measure *= self.dim[i]);
                measure
            }
            #[must_use]
            /// Returns the external boundary, the sum of the extents.
            ///
            /// It's equivalent to 2, perimeter and surface area in 1, 2 and 3 dimensions.
            pub const fn boundary(self) -> $f {
                let mut boundary = 0.0;
                whilst!(i in 0..D; {
                    let mut face_measure = 1.0;
                    whilst!(j in 0..D; is![i != j, face_measure *= self.dim[j]]);
                    boundary += face_measure;
                });
                2.0 * boundary // Each dimension's contribution is counted twice
            }
        }

        impl Extent2<$f> {
            #[must_use]
            /// Returns the area of the 2d extent.
            pub const fn area(self) -> $f { self.dim[0] * self.dim[1] }
            #[must_use]
            /// Returns the perimeter of the 2d extent.
            pub const fn perimeter(self) -> $f { 2.0 * (self.dim[0] + self.dim[1]) }
        }
        impl Extent3<$f> {
            #[must_use]
            /// Returns the volume of the 3d extent.
            pub const fn volume(self) -> $f {
                self.dim[0] * self.dim[1] * self.dim[2]
            }
            #[must_use]
            /// The surface area of the 3d extent.
            pub const fn surface_area(self) -> $f {
                2.0 * (self.dim[0] * self.dim[1]
                    + self.dim[1] * self.dim[2]
                    + self.dim[2] * self.dim[0])
            }
        }
    };
}
impl_extent![];

#[rustfmt::skip]
/// 1D accessors
impl<T: Copy> Extent1<T> {
    #[must_use]
    /// Returns a copy of the first dimension.
    pub const fn length(self) -> T { self.dim[0] }
    /// Returns a copy of the first dimension.
    pub const fn l(self) -> T { self.dim[0] }
}

#[rustfmt::skip]
/// 2D Accessors
impl<T: Copy> Extent2<T> {
    #[must_use]
    /// Returns a copy of the horizontal dimension (X-axis).
    pub const fn width(self) -> T { self.dim[0] }
    #[must_use]
    /// Returns a copy of the horizontal dimension (X-axis).
    pub const fn w(self) -> T { self.dim[0] }
    #[must_use]
    /// Returns a copy of the vertical dimension (Y-axis).
    pub const fn height(self) -> T { self.dim[1] }
    #[must_use]
    /// Returns a copy of the vertical dimension (Y-axis).
    pub const fn h(self) -> T { self.dim[1] }

    #[must_use]
    /// Returns a copy of the horizontal dimension (X-axis) (width).
    pub const fn length(self) -> T { self.dim[0] }
    #[must_use]
    /// Returns a copy of the horizontal dimension (X-axis) (width).
    pub const fn l(self) -> T { self.dim[0] }
    #[must_use]
    /// Returns a copy of the vertical dimension (Y-axis) (width).
    pub const fn breadth(self) -> T { self.dim[1] }
    #[must_use]
    /// Returns a copy of the vertical dimension (Y-axis) (width).
    pub const fn b(self) -> T { self.dim[1] }
}

#[rustfmt::skip]
/// 3D Accessors
impl<T: Copy> Extent3<T> {
    #[must_use]
    /// Returns a copy of the horizontal dimension (X-axis) (width).
    pub const fn width(self) -> T { self.dim[0] }
    #[must_use]
    /// Returns a copy of the horizontal dimension (X-axis) (width).
    pub const fn w(self) -> T { self.dim[0] }
    #[must_use]
    /// Returns a copy of the vertical dimension (Y-axis).
    pub const fn height(self) -> T { self.dim[1] }
    #[must_use]
    /// Returns a copy of the vertical dimension (Y-axis).
    pub const fn h(self) -> T { self.dim[1] }
    #[must_use]
    /// Returns a copy of the depth dimension (Z-axis).
    pub const fn depth(self) -> T { self.dim[2] }
    #[must_use]
    /// Returns a copy of the depth dimension (Z-axis).
    pub const fn d(self) -> T { self.dim[2] }
}

#[cfg(test)]
mod tests {
    use crate::{Extent, Extent2, Extent3, ext};

    #[test]
    fn test_macro_constructors() {
        let e = ext!([3; 8]);
        assert_eq![e.dim, [3, 3, 3, 3, 3, 3, 3, 3]];

        assert_eq![Extent::<i32, 1>::new([2]), ext!(2)];
        assert_eq![Extent2::<i32>::new([2, 5]), ext!(2, 5)];
        assert_eq![Extent3::<i32>::new([2, 5, 6]), ext!(2, 5, 6)];
        assert_eq![Extent::<i32, 4>::new([2, 5, 6, 7]), ext!(2, 5, 6, 7)];
    }

    #[test]
    fn test_macro_checked_casts() {
        let a = ext!(2_i32, 5);

        let b = ext!(checked => u8; a.x(), a.y());
        assert_eq![b, Ok(Extent2::new([2_u8, 5]))];

        let c = ext!(checked a => u8);
        assert_eq![c, Ok(Extent2::new([2_u8, 5]))];
    }

    #[test]
    fn test_macro_saturating_and_wrapping() {
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
    fn test_runtime_conversion_methods() {
        let a = ext!(300_u16, 40_u16);

        let _b: Extent2<u32> = a.map_into();
        let _c: Result<Extent2<u8>, _> = a.try_map_into();
        let _d = a.map(|x| x as f32 * 0.5);
        let _e: Result<Extent2<u8>, _> = a.try_map(u8::try_from);
    }
}
