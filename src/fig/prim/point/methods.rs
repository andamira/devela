// devela::fig::prim::point::methods
//
//!
//

use super::{Point, Point2d, Point3d, Vector};

impl<T, const D: usize> Point<T, D> {
    /// Returns a new `Point` from the given `array`.
    pub const fn new(array: [T; D]) -> Self {
        Self {
            coords: Vector::new(array),
        }
    }
}

/* manual impls for specific dimensionalities */

#[rustfmt::skip]
impl<T> Point2d<T> {
    /// Returns a copy of the first dimension `x`.
    #[inline] pub const fn x(self) -> T where T: Copy { self.coords.array[0] }
    /// Returns a copy of the second dimension `y`.
    #[inline] pub const fn y(self) -> T where T: Copy { self.coords.array[1] }
    /// Returns a shared reference to the first dimension `x`.
    #[inline] pub const fn x_ref(&self) -> &T { &self.coords.array[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[inline] pub const fn y_ref(&self) -> &T { &self.coords.array[1] }
    /// Returns an exclusive reference to the first dimension `x`.
    #[inline] pub fn x_mut(&mut self) -> &mut T { &mut self.coords.array[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[inline] pub fn y_mut(&mut self) -> &mut T { &mut self.coords.array[1] }
}

#[rustfmt::skip]
impl<T> Point3d<T> {
    /// Returns a copy of the first dimension `x`.
    #[inline] pub const fn x(self) -> T where T: Copy { self.coords.array[0] }
    /// Returns a copy of the second dimension `y`.
    #[inline] pub const fn y(self) -> T where T: Copy { self.coords.array[1] }
    /// Returns a copy of the third dimension `z`.
    #[inline] pub const fn z(self) -> T where T: Copy { self.coords.array[2] }

    /// Returns a shared reference to the first dimension `x`.
    #[inline] pub const fn x_ref(&self) -> &T { &self.coords.array[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[inline] pub const fn y_ref(&self) -> &T { &self.coords.array[1] }
    /// Returns a shared reference to the third dimension `z`.
    #[inline] pub const fn z_ref(&self) -> &T { &self.coords.array[2] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[inline] pub fn x_mut(&mut self) -> &mut T { &mut self.coords.array[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[inline] pub fn y_mut(&mut self) -> &mut T { &mut self.coords.array[1] }
    /// Returns an exclusive reference to the third dimension `z`.
    #[inline] pub fn z_mut(&mut self) -> &mut T { &mut self.coords.array[2] }
}

macro_rules! impl_point {
    // integers common methods
    //
    // $t: the inner integer primitive type
    (int $($t:ty),+) => { $( impl_point![@int $t]; )+ };
    (@int $t:ty) => {
        impl<const D: usize> Point<$t, D> {
            /// Adds the given vector.
            #[inline]
            pub const fn c_add_vector(self, v: Vector<$t, D>) -> Self {
                Self { coords: self.coords.c_add(v) }
            }
            /// Subtracts the given vector.
            #[inline]
            pub const fn c_sub_vector(self, v: Vector<$t, D>) -> Self {
                Self { coords: self.coords.c_sub(v) }
            }

        }
    };
    (sint $($t:ty),+) => { $( impl_point![@sint $t]; )+ };
    (@sint $t:ty ) => {
        impl_point![int $t];
    };
    (uint $($t:ty),+) => { $( impl_point![@uint $t]; )+ };
    (@uint $t:ty ) => {
        impl_point![int $t];
    };

    // $f: the inner floating-point primitive type
    (float $($f:ty),+) => { $( impl_point![@float $f]; )+ };
    (@float $f:ty) => {
        impl<const D: usize> Point<$f, D> {
            /// Adds the given vector.
            #[inline]
            pub fn add_vector(self, v: Vector<$f, D>) -> Self {
                Self { coords: self.coords.add(v) }
            }
            /// Subtracts the given vector.
            #[inline]
            pub fn sub_vector(self, v: Vector<$f, D>) -> Self {
                Self { coords: self.coords.sub(v) }
            }
        }
    };
}
impl_point![sint i8, i16, i32, i64, i128, isize];
impl_point![uint u8, u16, u32, u64, u128, usize];
impl_point![float f32, f64];
