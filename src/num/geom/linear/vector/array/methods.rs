// devela::num::geom::linear::vector::array::methods
//
//! impl methods for Vector
//

#[allow(unused_imports)]
#[cfg(not(feature = "std"))]
use crate::ExtFloat;
use crate::{Int, Vector, concat as cc, stringify as fy, unwrap};

/* common methods */

impl<T, const D: usize> Vector<T, D> {
    /// Returns a new `Vector` from the given `coords` array.
    pub const fn new(coords: [T; D]) -> Self {
        Self { coords }
    }
}

/* compile-time ops for primitives */

/// helper for implementing methods on `Vector`.
///
/// $t: the inner integer primitive type
macro_rules! impl_vector {
    () => {
        impl_vector![sint i8, i16, i32, i64, i128, isize];
        impl_vector![uint u8, u16, u32, u64, u128, usize];
        impl_vector![float f32, f64];
    };

    // integers common methods
    (int $($t:ty),+ $(,)?) => {
        $( impl_vector![@int $t]; )+
    };
    (@int $t:ty) => {
        #[doc = cc!("# Methods for vectors represented using `", fy!($t), "`.")]
        impl<const D: usize> Vector<$t, D> {
            /// A `Vector` with all ones.
            pub const ONE: Self = Self::new([1; D]);

            /// A `Vector` with all zeros.
            pub const ZERO: Self = Self::new([0; D]);

            /* ops with vector */

            /// Returns the normalized vector, using the given vector `magnitude`.
            ///
            /// $$
            /// \bm{n} = \widehat{\bm{a}} = \frac{1}{d}\thinspace\bm{a} =
            /// \frac{\bm{a}}{|\bm{a}|}
            /// $$
            pub const fn c_normalize_with(self, magnitude: $t) -> Self {
                let mut normalized = [0; D];
                let mut i = 0;
                while i < D {
                    normalized[i] = self.coords[i] / magnitude;
                    i += 1;
                }
                Vector { coords: normalized }
            }

            /// Calculates the magnitude of the vector (squared).
            ///
            /// This is faster than calculating the magnitude,
            /// which is useful for comparisons.
            ///
            /// # Formula
            /// $$ \large |\vec{V}|^2 = V_0^2 + ... + V_n^2 $$
            pub const fn c_magnitude_sq(self) -> $t { self.c_dot(self) }

            /// Adds two vectors together, in compile-time.
            pub const fn c_add(self, other: Self) -> Self {
                let mut result = [0; D];
                let mut i = 0;
                while i < D {
                    result[i] = self.coords[i] + other.coords[i];
                    i += 1;
                }
                Vector::new(result)
            }

            /// Subtracts another vector from this vector, in compile-time.
            pub const fn c_sub(self, other: Self) -> Self {
                let mut result = [0; D];
                let mut i = 0;
                while i < D {
                    result[i] = self.coords[i] - other.coords[i];
                    i += 1;
                }
                Vector::new(result)
            }

            /// Computes the dot product of two vectors, in compile-time.
            pub const fn c_dot(self, other: Self) -> $t {
                let mut result = 0;
                let mut i = 0;
                while i < D {
                    result += self.coords[i] * other.coords[i];
                    i += 1;
                }
                result
            }

            /* ops with scalar */

            /// Multiplies each element of the vector by a scalar, in compile-time.
            pub const fn c_scalar_mul(self, scalar: $t) -> Self {
                let mut result = [0; D];
                let mut i = 0;
                while i < D {
                    result[i] = self.coords[i] * scalar;
                    i += 1;
                }
                Vector::new(result)
            }

            /// Divides each element of the vector by a scalar, in compile-time.
            pub const fn c_scalar_div(self, scalar: $t) -> Self {
                let mut result = [0; D];
                let mut i = 0;
                while i < D {
                    result[i] = self.coords[i] / scalar;
                    i += 1;
                }
                Vector::new(result)
            }
        }

        #[doc = cc!("# Methods for 3d vectors represented using `", fy!($t), "`.")]
        impl Vector<$t, 3> {
            /// Computes the cross product of two vectors.
            ///
            /// That is the vector orthogonal to both vectors.
            ///
            /// Also known as the *exterior product* or the *vector product*.
            ///
            /// It is only defined for 3-dimensional vectors, and it is not
            /// commutative: $\vec{a}\times\vec{b} = -(\vec{b}\times\vec{a})$.
            ///
            /// # Formula
            /// $$
            /// \bm{a} \times \bm{b} =
            /// \begin{bmatrix} a_x \cr a_y \cr a_z \end{bmatrix} \times
            /// \begin{bmatrix} b_x \cr b_y \cr b_z \end{bmatrix} =
            /// \begin{bmatrix}
            ///     a_y b_z - a_z b_y \cr
            ///     a_z b_x - a_x b_z \cr
            ///     a_x b_y - a_y b_x
            /// \end{bmatrix}
            /// $$
            pub const fn c_cross(self, other: Self) -> Self {
                let cross_product = [
                    self.coords[1] * other.coords[2] - self.coords[2] * other.coords[1], // i
                    self.coords[2] * other.coords[0] - self.coords[0] * other.coords[2], // j
                    self.coords[0] * other.coords[1] - self.coords[1] * other.coords[0], // k
                ];
                Vector::new(cross_product)
            }
        }
    };

    // signed integers specific methods
    (sint $($t:ty),+ $(,)?) => {
        $( impl_vector![@sint $t]; )+
    };
    (@sint $t:ty) => {
        impl_vector![int $t];

        #[doc = cc!("# Methods for vectors represented using `", fy!($t), "`, signed.")]
        impl<const D: usize> Vector<$t, D> {
            /// A `Vector` with all negative ones.
            pub const NEG_ONE: Self = Self::new([-1; D]);

            /// Calculates the floored magnitude of the vector.
            ///
            /// It could underestimate the true magnitude.
            pub const fn c_magnitude_floor(self) -> $t {
                unwrap![ok Int(self.c_dot(self).abs()).sqrt_floor()].0
            }

            /// Calculates the ceiled magnitude of the vector.
            ///
            /// It could overestimate the true magnitude.
            pub const fn c_magnitude_ceil(self) -> $t {
                unwrap![ok Int(self.c_dot(self).abs()).sqrt_ceil()].0
            }

            /// Calculates the rounded magnitude of the vector.
            /// # Panics
            /// Can panic if we reach a `i128` value close to its maximum during operations.
            pub const fn c_magnitude_round(self) -> $t {
                unwrap![ok Int(self.c_dot(self).abs()).sqrt_round()].0
            }
        }
    };

    // unsigned integers specific methods
    (uint $($t:ty),+ $(,)?) => {
        $( impl_vector![@uint $t]; )+
    };
    (@uint $t:ty) => {
        impl_vector![int $t];

        #[doc = cc!("# Methods for vectors represented using `", fy!($t), "`, unsigned.")]
        impl<const D: usize> Vector<$t, D> {
            /// Calculates the floored magnitude of the vector.
            ///
            /// It could underestimate the true magnitude.
            pub const fn c_magnitude_floor(self) -> $t {
                Int(self.c_dot(self)).sqrt_floor().0
            }

            /// Calculates the ceiled magnitude of the vector.
            ///
            /// It could overestimate the true magnitude.
            pub const fn c_magnitude_ceil(self) -> $t {
                Int(self.c_dot(self)).sqrt_ceil().0
            }

            /// Calculates the rounded magnitude of the vector.
            /// # Panics
            /// Can panic if we reach a `u128` value close to its maximum during operations.
            pub const fn c_magnitude_round(self) -> $t {
                unwrap![ok Int(self.c_dot(self)).sqrt_round()].0
            }
        }
    };

    // $f: the inner floating-point primitive type
    (float $($f:ty),+ $(,)?) => {
        $( impl_vector![@float $f]; )+
    };
    (@float $f:ty) => {
        #[doc = cc!("# Methods for vectors represented using `", fy!($f), "`.")]
        impl<const D: usize> Vector<$f, D> {
            /// A `Vector` with all ones.
            pub const ONE: Self = Self::new([1.0; D]);
            /// A `Vector` with all zeros.
            pub const ZERO: Self = Self::new([0.0; D]);
            /// A `Vector` with all negative ones.
            pub const NEG_ONE: Self = Self::new([-1.0; D]);

            /// Returns the normalized vector, as a *unit vector*.
            ///
            /// $$
            /// \bm{n} = \widehat{\bm{a}} = \frac{1}{d}\thinspace\bm{a} =
            /// \frac{\bm{a}}{|\bm{a}|}
            /// $$
            pub fn normalize(&self) -> Self {
                let mag = self.magnitude();
                let mut normalized = [0.0; D];
                for i in 0..D {
                    normalized[i] = self.coords[i] / mag;
                }
                Vector { coords: normalized }
            }

            /// Calculates the magnitude of the vector.
            ///
            /// # Formula
            /// $$ \large |\vec{V}| = \sqrt{V_0^2 + ... + V_n^2} $$
            pub fn magnitude(self) -> $f { self.dot(self).sqrt() }

            /// Calculates the squared magnitude of the vector.
            ///
            /// This is faster than calculating the magnitude,
            /// which is useful for comparisons.
            ///
            /// # Formula
            /// $$ \large |\vec{V}|^2 = V_0^2 + ... + V_n^2 $$
            pub fn magnitude_sq(self) -> $f { self.dot(self) }

            /// Adds two vectors together.
            #[allow(clippy::should_implement_trait)]
            pub fn add(self, other: Self) -> Self {
                let mut result = [0.0; D];
                let mut i = 0;
                while i < D {
                    result[i] = self.coords[i] + other.coords[i];
                    i += 1;
                }
                Vector::new(result)
            }

            /// Subtracts another vector from this vector.
            #[allow(clippy::should_implement_trait)]
            pub fn sub(self, other: Self) -> Self {
                let mut result = [0.0; D];
                let mut i = 0;
                while i < D {
                    result[i] = self.coords[i] - other.coords[i];
                    i += 1;
                }
                Vector::new(result)
            }

            /// Computes the dot product of two vectors.
            ///
            /// That is the magnitude of one vector in the direction of another.
            ///
            /// Also known as the *inner produc* or the *scalar product*.
            ///
            /// # Formula
            /// $$
            /// \large \vec{a}\cdot\vec{b} =
            /// \begin{bmatrix} a_0 \cr ... \cr a_n \end{bmatrix} \cdot
            /// \begin{bmatrix} b_0 \cr ... \cr b_n \end{bmatrix} =
            /// a_0 b_0 + ... + a_n b_n
            /// $$
            pub fn dot(self, other: Self) -> $f {
                let mut result = 0.0;
                let mut i = 0;
                while i < D {
                    result += self.coords[i] * other.coords[i];
                    i += 1;
                }
                result
            }
        }

        #[doc = cc!("# Methods for 3d vectors represented using `", fy!($f), "`.")]
        impl Vector<$f, 3> {
            /// Computes the cross product of two vectors.
            ///
            /// That is the vector orthogonal to both vectors.
            ///
            /// Also known as the *exterior product* or the *vector product*.
            ///
            /// It is only defined for 3-dimensional vectors, and it is not
            /// commutative: $\vec{a}\times\vec{b} = -(\vec{b}\times\vec{a})$.
            ///
            /// # Formula
            /// $$
            /// \bm{a} \times \bm{b} =
            /// \begin{bmatrix} a_x \cr a_y \cr a_z \end{bmatrix} \times
            /// \begin{bmatrix} b_x \cr b_y \cr b_z \end{bmatrix} =
            /// \begin{bmatrix}
            ///     a_y b_z - a_z b_y \cr
            ///     a_z b_x - a_x b_z \cr
            ///     a_x b_y - a_y b_x
            /// \end{bmatrix}
            /// $$
            pub fn cross(self, other: Self) -> Self {
                let cross_product = [
                    self.coords[1] * other.coords[2] - self.coords[2] * other.coords[1], // i
                    self.coords[2] * other.coords[0] - self.coords[0] * other.coords[2], // j
                    self.coords[0] * other.coords[1] - self.coords[1] * other.coords[0], // k
                ];
                Vector::new(cross_product)
            }
        }
    };
}
impl_vector!();
