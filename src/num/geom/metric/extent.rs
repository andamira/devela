// devela::num::geom::metric::extent
//
//! A geometrical extent.
//

use crate::{_impl_metric, cfor, is};

#[doc = crate::_TAG_GEOM!()]
/// An orthogonal extension in `D`-space without a coordinate position.
///
/// Represents the lengths of each dimension in a multi-dimensional space,
/// providing an origin-agnostic shape with the implied form of an orthotope
/// (generalized rectangle or box).
#[must_use]
#[repr(transparent)]
pub struct Extent<T, const D: usize> {
    /// The size values in `D`-dimensional space.
    pub dim: [T; D],
}

_impl_metric![common_methods: Extent];
_impl_metric![common_traits: Extent];

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
            /// Returns the internal measure, the product of the extents.
            ///
            /// It's equivalent to length, area, and volume in 1, 2 and 3 dimensions.
            pub const fn c_measure(self) -> $t {
                let mut measure = 1;
                cfor!(i in 0..D => {
                    measure *= self.dim[i];
                });
                measure
            }
            /// Returns the external boundary, the sum of the extents.
            ///
            /// It's equivalent to 2, perimeter and surface area in 1, 2 and 3 dimensions.
            pub const fn c_boundary(self) -> $t {
                let mut boundary = 0;
                cfor!(i in 0..D => {
                    let mut face_measure = 1;
                    cfor!(j in 0..D => {
                        is![i != j; face_measure *= self.dim[j]];
                    });
                    boundary += face_measure;
                });
                2 * boundary // Each dimension's contribution is counted twice
            }
        }

        impl Extent<$t, 1> {
            /// The length of the 1d extent.
            #[must_use]
            pub const fn c_length(self) -> $t { self.dim[0] }
        }
        impl Extent<$t, 2> {
            /// The area of the 2d extent.
            #[must_use]
            pub const fn c_area(self) -> $t { self.dim[0] * self.dim[1] }

            /// The perimeter of the 2d extent.
            #[must_use]
            pub const fn c_perimeter(self) -> $t { 2 * (self.dim[0] + self.dim[1]) }

        }
        impl Extent<$t, 3> {
            /// The volume of the 3d extent.
            #[must_use]
            pub const fn c_volume(self) -> $t {
                self.dim[0] * self.dim[1] * self.dim[2]
            }

            /// The surface area of the 3d extent.
            #[must_use]
            pub const fn c_surface_area(self) -> $t {
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
            /// Returns the internal measure, the product of the extents.
            ///
            /// It's equivalent to length, area, and volume in 1, 2 and 3 dimensions.
            #[must_use]
            pub const fn measure(self) -> $f {
                let mut measure = 1.0;
                cfor!(i in 0..D => {
                    measure *= self.dim[i];
                });
                measure
            }
            /// Returns the external boundary, the sum of the extents.
            ///
            /// It's equivalent to 2, perimeter and surface area in 1, 2 and 3 dimensions.
            #[must_use]
            pub const fn boundary(self) -> $f {
                let mut boundary = 0.0;
                cfor!(i in 0..D => {
                    let mut face_measure = 1.0;
                    cfor!(j in 0..D => {
                        is![i != j; face_measure *= self.dim[j]];
                    });
                    boundary += face_measure;
                });
                2.0 * boundary // Each dimension's contribution is counted twice
            }
        }

        impl Extent<$f, 1> {
            /// The length of the 1d extent.
            #[must_use]
            pub const fn length(self) -> $f { self.dim[0] }
        }
        impl Extent<$f, 2> {
            /// The area of the 2d extent.
            #[must_use]
            pub const fn area(self) -> $f { self.dim[0] * self.dim[1] }

            /// The perimeter of the 2d extent.
            #[must_use]
            pub const fn perimeter(self) -> $f { 2.0 * (self.dim[0] + self.dim[1]) }

        }
        impl Extent<$f, 3> {
            /// The volume of the 3d extent.
            #[must_use]
            pub const fn volume(self) -> $f {
                self.dim[0] * self.dim[1] * self.dim[2]
            }

            /// The surface area of the 3d extent.
            #[must_use]
            pub const fn surface_area(self) -> $f {
                2.0 * (self.dim[0] * self.dim[1]
                    + self.dim[1] * self.dim[2]
                    + self.dim[2] * self.dim[0])
            }
        }
    };
}
impl_extent![];
