// devela::geom::shape::point::methods
//
//!
//
// TOC
// - Point
// - Point2d
// - Point3d
// - impl_point!
// - Points

#[cfg(feature = "linear")]
use crate::Vector;
use crate::{Point, Point2d, Point3d, Points};

#[rustfmt::skip]
impl<T, const D: usize> Point<T, D> {
    #[must_use] /// Returns a new `Point` from the given `coords` array.
    pub const fn new(coords: [T; D]) -> Self { Self { coords } }

    #[must_use] /// Consumes this `Point` and converts it into a `Vector`.
    #[cfg(feature = "linear")] #[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
    pub fn into_vector(self) -> Vector<T, D> { Vector::new(self.coords) }
    #[must_use] /// Converts this `Point` to a `Vector`, keeping `self` intact.
    #[cfg(feature = "linear")] #[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
    pub const fn to_vector(self) -> Vector<T, D> where T: Copy { Vector::new(self.coords) }

    #[must_use] /// Creates a `Point` from a `Vector`.
    #[cfg(feature = "linear")] #[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
    pub fn from_vector(v: Vector<T, D>) -> Self { Self::new(v.coords) }
    #[must_use] /// Creates a `Point` from a constant `Vector`.
    #[cfg(feature = "linear")] #[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
    pub const fn from_vector_const(v: Vector<T, D>) -> Self where T: Copy { Self::new(v.coords) }
}

/* manual impls for specific dimensionalities */

#[rustfmt::skip]
impl<T> Point2d<T> {
    /// Returns a copy of the first dimension `x`.
    #[must_use] pub const fn x(self) -> T where T: Copy { self.coords[0] }
    /// Returns a copy of the second dimension `y`.
    #[must_use] pub const fn y(self) -> T where T: Copy { self.coords[1] }

    /// Returns a shared reference to the first dimension `x`.
    #[must_use] pub const fn x_ref(&self) -> &T { &self.coords[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[must_use] pub const fn y_ref(&self) -> &T { &self.coords[1] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[must_use] pub const fn x_mut(&mut self) -> &mut T { &mut self.coords[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[must_use] pub const fn y_mut(&mut self) -> &mut T { &mut self.coords[1] }
}

#[rustfmt::skip]
impl<T> Point3d<T> {
    /// Returns a copy of the first dimension `x`.
    #[must_use] pub const fn x(self) -> T where T: Copy { self.coords[0] }
    /// Returns a copy of the second dimension `y`.
    #[must_use] pub const fn y(self) -> T where T: Copy { self.coords[1] }
    /// Returns a copy of the third dimension `z`.
    #[must_use] pub const fn z(self) -> T where T: Copy { self.coords[2] }

    /// Returns a shared reference to the first dimension `x`.
    #[must_use] pub const fn x_ref(&self) -> &T { &self.coords[0] }
    /// Returns a shared reference to the second dimension `y`.
    #[must_use] pub const fn y_ref(&self) -> &T { &self.coords[1] }
    /// Returns a shared reference to the third dimension `z`.
    #[must_use] pub const fn z_ref(&self) -> &T { &self.coords[2] }

    /// Returns an exclusive reference to the first dimension `x`.
    #[must_use] pub const fn x_mut(&mut self) -> &mut T { &mut self.coords[0] }
    /// Returns an exclusive reference to the second dimension `y`.
    #[must_use] pub const fn y_mut(&mut self) -> &mut T { &mut self.coords[1] }
    /// Returns an exclusive reference to the third dimension `z`.
    #[must_use] pub const fn z_mut(&mut self) -> &mut T { &mut self.coords[2] }
}

/// helper for implementing methods on `Point`.
///
/// $t: the inner integer primitive type
macro_rules! impl_point {
    () => {
        impl_point![sint i8, i16, i32, i64, i128, isize];
        impl_point![uint u8, u16, u32, u64, u128, usize];
        impl_point![float f32:"_float_f32", f64:"_float_f64"];
    };

    // integers common methods
    (int $($t:ty),+) => { $( impl_point![@int $t]; )+ };
    (@int $t:ty) => {
        impl<const D: usize> Point<$t, D> {
            /// Adds the given vector.
            #[must_use]
            #[cfg(feature = "linear")]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
            pub const fn c_add_vector(self, v: Vector<$t, D>) -> Self {
                Self { coords: Vector::new(self.coords).c_add(v).coords }
            }
            // /// Subtracts the given vector.
            //
            // #[cfg(feature = "linear")]
            // #[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
            // pub const fn c_sub_vector(self, v: Vector<$t, D>) -> Self {
            //     Self { coords: self.coords.c_sub(v) }
            // }
        }
    };
    (sint $($t:ty),+) => { $( impl_point![@sint $t]; )+ };
    (@sint $t:ty) => {
        impl_point![int $t];
    };
    (uint $($t:ty),+) => { $( impl_point![@uint $t]; )+ };
    (@uint $t:ty) => {
        impl_point![int $t];
    };

    // $f: the inner floating-point primitive type
    (float $( $f:ty : $cap:literal ),+) => { $( impl_point![@float $f:$cap]; )+ };
    (@float $f:ty : $cap:literal) => {
        impl<const D: usize> Point<$f, D> {
            // TODO
            //
            // /// Adds the given vector.
            //
            // #[cfg(feature = "linear")]
            // #[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
            // pub fn add_vector(self, v: Vector<$f, D>) -> Self {
            //     Self { coords: self.coords.add(v) }
            // }
            // /// Subtracts the given vector.
            //
            // #[cfg(feature = "linear")]
            // #[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
            // pub fn sub_vector(self, v: Vector<$f, D>) -> Self {
            //     Self { coords: self.coords.sub(v) }
            // }
        }
    };
}
use impl_point;
impl_point!();

#[rustfmt::skip]
impl<T, const D: usize, const N: usize> Points<T, D, N> {
    /// Returns new `Points` from the given `coords` array.
    pub const fn new(points: [Point<T, D>; N]) -> Self { Self { array: points } }
}
