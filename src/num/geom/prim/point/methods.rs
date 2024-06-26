// devela::num::geom::prim::point::methods
//
//!
//

use super::{Point, Point2d, Point3d};

// use crate::num::geom::Vector;

impl<T, const D: usize> Point<T, D> {
    /// Returns a new `Point` from the given `array`.
    pub const fn new(array: [T; D]) -> Self {
        Self { coords: array }
    }
}

/* manual impls for specific dimensionalities */

#[rustfmt::skip]
impl<T> Point2d<T> {
    /// Returns a copy of the first dimension `x`.
    #[inline] pub const fn x(self) -> T where T: Copy { self.coords[0] }
    /// Returns a copy of the second dimension `y`.
    #[inline] pub const fn y(self) -> T where T: Copy { self.coords[1] }
    /// Returns a shared reference to the first dimension `x`.
    #[inline] pub const fn x_ref(&self) -> &T { &self.coords[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[inline] pub const fn y_ref(&self) -> &T { &self.coords[1] }
    /// Returns an exclusive reference to the first dimension `x`.
    #[inline] pub fn x_mut(&mut self) -> &mut T { &mut self.coords[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[inline] pub fn y_mut(&mut self) -> &mut T { &mut self.coords[1] }
}

#[rustfmt::skip]
impl<T> Point3d<T> {
    /// Returns a copy of the first dimension `x`.
    #[inline] pub const fn x(self) -> T where T: Copy { self.coords[0] }
    /// Returns a copy of the second dimension `y`.
    #[inline] pub const fn y(self) -> T where T: Copy { self.coords[1] }
    /// Returns a copy of the third dimension `z`.
    #[inline] pub const fn z(self) -> T where T: Copy { self.coords[2] }

    /// Returns a shared reference to the first dimension `x`.
    #[inline] pub const fn x_ref(&self) -> &T { &self.coords[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[inline] pub const fn y_ref(&self) -> &T { &self.coords[1] }
    /// Returns a shared reference to the third dimension `z`.
    #[inline] pub const fn z_ref(&self) -> &T { &self.coords[2] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[inline] pub fn x_mut(&mut self) -> &mut T { &mut self.coords[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[inline] pub fn y_mut(&mut self) -> &mut T { &mut self.coords[1] }
    /// Returns an exclusive reference to the third dimension `z`.
    #[inline] pub fn z_mut(&mut self) -> &mut T { &mut self.coords[2] }
}

// $t: the inner integer primitive type
// $cap:  the capability feature that enables the given implementation. E.g "_int_i8".
macro_rules! impl_point {
    () => {
        impl_point![sint i8:"_int_i8", i16:"_int_i16", i32:"_int_i32",
            i64:"_int_i64", i128:"_int_i128", isize:"_int_isize"];
        impl_point![uint u8:"_int_u8", u16:"_int_u16", u32:"_int_u32",
            u64:"_int_u64", u128:"_int_u128", usize:"_int_usize"];
        impl_point![float f32:"_float_f32", f64:"_float_f64"];
    };

    // integers common methods
    (int $($t:ty : $cap:literal),+) => { $( impl_point![@int $t:$cap]; )+ };
    (@int $t:ty : $cap:literal) => {
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<const D: usize> Point<$t, D> {
            // TODO
            //
            // /// Adds the given vector.
            // #[inline]
            // pub const fn c_add_vector(self, v: Vector<$t, D>) -> Self {
            //     Self { coords: self.coords.c_add(v) }
            // }
            // /// Subtracts the given vector.
            // #[inline]
            // pub const fn c_sub_vector(self, v: Vector<$t, D>) -> Self {
            //     Self { coords: self.coords.c_sub(v) }
            // }
        }
    };
    (sint $($t:ty : $cap:literal),+) => { $( impl_point![@sint $t:$cap]; )+ };
    (@sint $t:ty : $cap:literal ) => {
        impl_point![int $t:$cap];

        // #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
    };
    (uint $($t:ty : $cap:literal),+) => { $( impl_point![@uint $t:$cap]; )+ };
    (@uint $t:ty : $cap:literal) => {
        impl_point![int $t:$cap];

        // #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
    };

    // $f: the inner floating-point primitive type
    (float $( $f:ty : $cap:literal ),+) => { $( impl_point![@float $f:$cap]; )+ };
    (@float $f:ty : $cap:literal) => {
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<const D: usize> Point<$f, D> {
            // TODO
            //
            // /// Adds the given vector.
            // #[inline]
            // pub fn add_vector(self, v: Vector<$f, D>) -> Self {
            //     Self { coords: self.coords.add(v) }
            // }
            // /// Subtracts the given vector.
            // #[inline]
            // pub fn sub_vector(self, v: Vector<$f, D>) -> Self {
            //     Self { coords: self.coords.sub(v) }
            // }
        }
    };
}
impl_point!();
