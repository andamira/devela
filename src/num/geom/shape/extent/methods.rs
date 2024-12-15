// devela::num::geom::shape::extent::methods
//
//!
//

use super::{Extent, Extent2d, Extent3d};
use crate::{cfor, iif};

#[rustfmt::skip]
impl<T, const D: usize> Extent<T, D> {
    /// Constructs a new `Extent` from the given dimensions.
    pub const fn new(dimensions: [T; D]) -> Self {
        Self { extent: dimensions }
    }

    /// Returns a shared reference to the extent as a slice.
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        &self.extent
    }
    /// Returns an exclusive reference to the extent as a slice.
    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.extent
    }

    /// Returns `true` if all dimensions of the extent are equal.
    #[must_use]
    pub fn is_uniform_nd(&self) -> bool where T: PartialEq {
        iif![D == 0; return true];
        let mut i = 1;
        while i < D {
            iif![self.extent[i] != self.extent[0]; return false];
            i += 1;
        }
        true
    }
}

/* manual impls for specific dimensionalities */

#[rustfmt::skip]
impl<T> Extent2d<T> {
    /// Returns a copy of the first dimension `x`.
    #[must_use]
    pub const fn x(self) -> T where T: Copy { self.extent[0] }
    /// Returns a copy of the second dimension `y`.
    #[must_use]
    pub const fn y(self) -> T where T: Copy { self.extent[1] }

    /// Returns a shared reference to the first dimension `x`.
    #[must_use]
    pub const fn x_ref(&self) -> &T { &self.extent[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[must_use]
    pub const fn y_ref(&self) -> &T { &self.extent[1] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[must_use]
    pub fn x_mut(&mut self) -> &mut T { &mut self.extent[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[must_use]
    pub fn y_mut(&mut self) -> &mut T { &mut self.extent[1] }

    /// Returns `true` if the 2 dimensions of the extent are equal.
    #[must_use]
    pub fn is_uniform(&self) -> bool where T: PartialEq {
        self.extent[0] == self.extent[1]
    }
}

#[rustfmt::skip]
impl<T> Extent3d<T> {
    /// Returns a copy of the first dimension `x`.
    #[must_use]
    pub const fn x(self) -> T where T: Copy { self.extent[0] }
    /// Returns a copy of the second dimension `y`.
    #[must_use]
    pub const fn y(self) -> T where T: Copy { self.extent[1] }
    /// Returns a copy of the third dimension `z`.
    #[must_use]
    pub const fn z(self) -> T where T: Copy { self.extent[2] }

    /// Returns a shared reference to the first dimension `x`.
    #[must_use]
    pub const fn x_ref(&self) -> &T { &self.extent[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[must_use]
    pub const fn y_ref(&self) -> &T { &self.extent[1] }
    /// Returns a shared reference to the third dimension `z`.
    #[must_use]
    pub const fn z_ref(&self) -> &T { &self.extent[2] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[must_use]
    pub fn x_mut(&mut self) -> &mut T { &mut self.extent[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[must_use]
    pub fn y_mut(&mut self) -> &mut T { &mut self.extent[1] }
    /// Returns an exclusive reference to the third dimension `z`.
    #[must_use]
    pub fn z_mut(&mut self) -> &mut T { &mut self.extent[2] }

    /// Returns `true` if the 3 dimensions of the extent are equal.
    #[must_use]
    pub fn is_uniform_3d(&self) -> bool where T: PartialEq {
        self.extent[0] == self.extent[1] && self.extent[0] == self.extent[2]
    }
}

/// Implement `Extent`.
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
                    measure *= self.extent[i];
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
                        iif![i != j; face_measure *= self.extent[j]];
                    });
                    boundary += face_measure;
                });
                2 * boundary // Each dimension's contribution is counted twice
            }
        }

        impl Extent<$t, 1> {
            /// The length of the 1d extent.
            #[must_use]
            pub const fn c_length(self) -> $t { self.extent[0] }
        }
        impl Extent2d<$t> {
            /// The area of the 2d extent.
            #[must_use]
            pub const fn c_area(self) -> $t { self.extent[0] * self.extent[1] }

            /// The perimeter of the 2d extent.
            #[must_use]
            pub const fn c_perimeter(self) -> $t { 2 * (self.extent[0] + self.extent[1]) }

        }
        impl Extent3d<$t> {
            /// The volume of the 3d extent.
            #[must_use]
            pub const fn c_volume(self) -> $t {
                self.extent[0] * self.extent[1] * self.extent[2]
            }

            /// The surface area of the 3d extent.
            #[must_use]
            pub const fn c_surface_area(self) -> $t {
                2 * (self.extent[0] * self.extent[1]
                    + self.extent[1] * self.extent[2]
                    + self.extent[2] * self.extent[0])
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
                    measure *= self.extent[i];
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
                        iif![i != j; face_measure *= self.extent[j]];
                    });
                    boundary += face_measure;
                });
                2.0 * boundary // Each dimension's contribution is counted twice
            }
        }

        impl Extent<$f, 1> {
            /// The length of the 1d extent.
            #[must_use]
            pub const fn length(self) -> $f { self.extent[0] }
        }
        impl Extent2d<$f> {
            /// The area of the 2d extent.
            #[must_use]
            pub const fn area(self) -> $f { self.extent[0] * self.extent[1] }

            /// The perimeter of the 2d extent.
            #[must_use]
            pub const fn perimeter(self) -> $f { 2.0 * (self.extent[0] + self.extent[1]) }

        }
        impl Extent3d<$f> {
            /// The volume of the 3d extent.
            #[must_use]
            pub const fn volume(self) -> $f {
                self.extent[0] * self.extent[1] * self.extent[2]
            }

            /// The surface area of the 3d extent.
            #[must_use]
            pub const fn surface_area(self) -> $f {
                2.0 * (self.extent[0] * self.extent[1]
                    + self.extent[1] * self.extent[2]
                    + self.extent[2] * self.extent[0])
            }
        }
    };
}
impl_extent![];
