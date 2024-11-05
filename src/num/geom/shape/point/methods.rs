// devela::geom::shape::point::methods
//
//!
//

use super::{Point, Point2d, Point3d};
use crate::num::algebra::linear::Vector;

#[rustfmt::skip]
impl<T, const D: usize> Point<T, D> {
    /// Returns a new `Point` from the given `coords` array.
    #[inline]
    #[must_use]
    pub const fn new(coords: [T; D]) -> Self {
        Self { coords }
    }

    /// Consumes this `Point` and converts it into a `Vector`.
    #[inline] #[must_use]
    pub fn into_vector(self) -> Vector<T, D> {
        Vector::new(self.coords)
    }
    /// Converts this `Point` to a `Vector`, keeping `self` intact.
    #[inline] #[must_use]
    pub const fn to_vector(self) -> Vector<T, D> where T: Copy {
        Vector::new(self.coords)
    }

    /// Creates a `Point` from a `Vector`.
    #[inline] #[must_use]
    pub fn from_vector(v: Vector<T, D>) -> Self {
        Self::new(v.coords)
    }
    /// Creates a `Point` from a constant `Vector`.
    #[inline] #[must_use]
    pub const fn from_vector_const(v: Vector<T, D>) -> Self where T: Copy {
        Self::new(v.coords)
    }
}

/* manual impls for specific dimensionalities */

#[rustfmt::skip]
impl<T> Point2d<T> {
    /// Returns a copy of the first dimension `x`.
    #[inline] #[must_use]
    pub const fn x(self) -> T where T: Copy { self.coords[0] }
    /// Returns a copy of the second dimension `y`.
    #[inline] #[must_use]
    pub const fn y(self) -> T where T: Copy { self.coords[1] }

    /// Returns a shared reference to the first dimension `x`.
    #[inline] #[must_use]
    pub const fn x_ref(&self) -> &T { &self.coords[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[inline] #[must_use]
    pub const fn y_ref(&self) -> &T { &self.coords[1] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[inline] #[must_use]
    pub fn x_mut(&mut self) -> &mut T { &mut self.coords[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[inline] #[must_use]
    pub fn y_mut(&mut self) -> &mut T { &mut self.coords[1] }
}

#[rustfmt::skip]
impl<T> Point3d<T> {
    /// Returns a copy of the first dimension `x`.
    #[inline] #[must_use]
    pub const fn x(self) -> T where T: Copy { self.coords[0] }
    /// Returns a copy of the second dimension `y`.
    #[inline] #[must_use]
    pub const fn y(self) -> T where T: Copy { self.coords[1] }
    /// Returns a copy of the third dimension `z`.
    #[inline] #[must_use]
    pub const fn z(self) -> T where T: Copy { self.coords[2] }

    /// Returns a shared reference to the first dimension `x`.
    #[inline] #[must_use]
    pub const fn x_ref(&self) -> &T { &self.coords[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[inline] #[must_use]
    pub const fn y_ref(&self) -> &T { &self.coords[1] }
    /// Returns a shared reference to the third dimension `z`.
    #[inline] #[must_use]
    pub const fn z_ref(&self) -> &T { &self.coords[2] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[inline] #[must_use]
    pub fn x_mut(&mut self) -> &mut T { &mut self.coords[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[inline] #[must_use]
    pub fn y_mut(&mut self) -> &mut T { &mut self.coords[1] }
    /// Returns an exclusive reference to the third dimension `z`.
    #[inline] #[must_use]
    pub fn z_mut(&mut self) -> &mut T { &mut self.coords[2] }
}

#[doc = crate::doc_private!()]
/// helper for implementing methods on `Point`.
///
/// $t: the inner integer primitive type
/// $cap:  the capability feature that enables the given implementation. E.g "_int_i8".
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
            /// Adds the given vector.
            #[inline] #[must_use]
            pub const fn c_add_vector(self, v: Vector<$t, D>) -> Self {
                Self { coords: Vector::new(self.coords).c_add(v).coords }
            }
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
