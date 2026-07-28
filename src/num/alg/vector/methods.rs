// devela/src/num/alg/vector/methods.rs
//
//! impl methods for Vector
//

#[cfg(feature = "int")]
use crate::Int;
use crate::{Float, NumConst, Vector, is, unwrap, whilst};

impl<T, const D: usize> Vector<T, D> {
    /// Returns a new `Vector` from the given `coords` array.
    pub const fn new(coords: [T; D]) -> Self {
        Self { coords }
    }
}

impl<T: Copy, const D: usize> Vector<T, D> {
    /// Returns a vector whose components are all `value`.
    pub const fn splat(value: T) -> Self {
        Self::new([value; D])
    }
}

/* compile-time ops for primitives */

/// Helper for implementing methods on `Vector`.
///
/// `$t`: the primitive component type.
macro_rules! _impl_vector {
    () => {
        _impl_vector![sint i8, i16, i32, i64, i128, isize];
        _impl_vector![uint u8, u16, u32, u64, u128, usize];
        _impl_vector![float f32, f64];
    };
    (@common $t:ty) => {
        impl<const D: usize> Vector<$t, D> {
            /// A `Vector` with all zeros.
            pub const ZERO: Self = Self::new([<$t>::NUM_ZERO.unwrap(); D]);

            /* ops with vector */

            /// Adds two vectors together.
            #[allow(clippy::should_implement_trait)]
            pub const fn add(self, other: Self) -> Self {
                let mut result = Self::ZERO;
                whilst! { i in 0..D; {
                    result.coords[i] = self.coords[i] + other.coords[i];
                }}
                result
            }

            /// Subtracts another vector from this vector.
            #[allow(clippy::should_implement_trait)]
            pub const fn sub(self, other: Self) -> Self {
                let mut result = Self::ZERO;
                whilst! { i in 0..D; {
                    result.coords[i] = self.coords[i] - other.coords[i];
                }}
                result
            }

            /// Computes the coordinate dot product of `self` and `other`.
            ///
            /// Its usual Euclidean geometric interpretation assumes
            /// that both vectors are expressed in the same orthonormal basis.
            ///
            /// Also known as the *inner product* or the *scalar product*.
            ///
            /// # Formula
            /// $$
            /// \large \vec{a}\cdot\vec{b} =
            /// \begin{bmatrix} a_0 \cr ... \cr a_n \end{bmatrix} \cdot
            /// \begin{bmatrix} b_0 \cr ... \cr b_n \end{bmatrix} =
            /// a_0 b_0 + ... + a_n b_n
            /// $$
            #[must_use]
            pub const fn dot(&self, other: &Self) -> $t {
                let mut result = <$t>::NUM_ZERO.unwrap();
                whilst! { i in 0..D; {
                    result += self.coords[i] * other.coords[i];
                }}
                result
            }

            /* ops with scalar */

            /// Multiplies each component by a scalar.
            pub const fn mul_scalar(self, scalar: $t) -> Self {
                let mut result = Self::ZERO;
                whilst! { i in 0..D; {
                    result.coords[i] = self.coords[i] * scalar;
                }}
                result
            }
        }
    };
    (@common_signed $t:ty) => {
        impl<const D: usize> Vector<$t, D> {
            /// Negates every component.
            ///
            /// Each component follows the negation semantics of its primitive type.
            pub const fn neg(mut self) -> Self {
                whilst! { i in 0..D; {
                    self.coords[i] = -self.coords[i];
                }}
                self
            }
        }

        /// Methods for 3d vectors.
        impl Vector<$t, 3> {
            /// Computes the three-dimensional cross product.
            ///
            /// In an oriented Euclidean 3-space, this corresponds
            /// to the Hodge dual of the exterior product.
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
            pub const fn cross(self, other: Self) -> Self {
                let cross_product = [
                    self.coords[1] * other.coords[2] - self.coords[2] * other.coords[1], // i
                    self.coords[2] * other.coords[0] - self.coords[0] * other.coords[2], // j
                    self.coords[0] * other.coords[1] - self.coords[1] * other.coords[0], // k
                ];
                Vector::new(cross_product)
            }
        }
    };

    // integers common methods
    (int $($t:ty),+ $(,)?) => {
        $( _impl_vector![@int $t]; )+
    };
    (@int $t:ty) => {
        _impl_vector![@common $t];

        impl<const D: usize> Vector<$t, D> {
            /// Divides every component by `scalar`.
            ///
            /// # Panics
            /// Panics if `scalar` is zero.
            ///
            /// For signed integers, it also panics if a component
            /// is the minimum representable value and `scalar` is `-1`.
            pub const fn div_scalar(self, scalar: $t) -> Self {
                assert!(scalar != 0, "attempt to divide a vector by zero");
                let mut result = Self::ZERO;
                whilst! { i in 0..D; {
                    result.coords[i] = self.coords[i] / scalar;
                }}
                result
            }
            /// Returns the component-wise quotient,
            /// or `None` if the scalar is zero or any component division overflows.
            #[must_use]
            pub const fn checked_div_scalar(self, scalar: $t) -> Option<Self> {
                is! { scalar == 0, return None }
                let mut result = Self::ZERO;
                whilst! { i in 0..D; {
                    result.coords[i] = unwrap![some? self.coords[i].checked_div(scalar)];
                }}
                Some(result)
            }
        }
    };

    // signed integers specific methods
    (sint $($t:ty),+ $(,)?) => {
        $( _impl_vector![@sint $t]; )+
    };
    (@sint $t:ty) => {
        _impl_vector![int $t];
        _impl_vector![@common_signed $t];

        impl<const D: usize> Vector<$t, D> {
            /// Returns the component-wise negation,
            /// or `None` if any component is the minimum representable value.
            #[must_use]
            pub const fn checked_neg(self) -> Option<Self> {
                let mut result = Self::ZERO;
                whilst! { i in 0..D; {
                    result.coords[i] = unwrap![some? self.coords[i].checked_neg()];
                }}
                Some(result)
            }
        }
    };

    // unsigned integers specific methods
    (uint $($t:ty),+ $(,)?) => {
        $( _impl_vector![@uint $t]; )+
    };
    (@uint $t:ty) => {
        _impl_vector![int $t];

        #[cfg(feature = "int")]
        impl<const D: usize> Vector<$t, D> {
            /// Calculates the floored magnitude of the vector.
            ///
            /// It can underestimate the true magnitude.
            ///
            /// # Overflow
            /// The sum of squared components follows the overflow semantics
            /// of the primitive component type.
            #[must_use]
            pub const fn magnitude_floor(&self) -> $t {
                Int(self.dot(self)).sqrt_floor().0
            }
            /// Calculates the ceiled magnitude of the vector.
            ///
            /// It could overestimate the true magnitude.
            #[must_use]
            pub const fn magnitude_ceil(&self) -> $t {
                Int(self.dot(self)).sqrt_ceil().0
            }
            /// Calculates the rounded magnitude of the vector.
            /// # Panics
            /// Can panic if we reach a `u128` value close to its maximum during operations.
            #[must_use]
            pub const fn magnitude_round(&self) -> $t {
                unwrap![ok Int(self.dot(self)).sqrt_round()].0
            }
        }
    };

    // $f: the inner floating-point primitive type
    (float $($f:ty),+ $(,)?) => {
        $( _impl_vector![@float $f]; )+
    };
    (@float $f:ty) => {
        _impl_vector![@common $f];
        _impl_vector![@common_signed $f];

        impl<const D: usize> Vector<$f, D> {
            /// Divides every component by `scalar`.
            ///
            /// Each component follows IEEE 754 floating-point division semantics.
            pub const fn div_scalar(self, scalar: $f) -> Self {
                let mut result = Self::ZERO;
                whilst! { i in 0..D; {
                    result.coords[i] = self.coords[i] / scalar;
                }}
                result
            }

            /// Returns the normalized vector, as a *unit vector*.
            ///
            /// $$
            /// \bm{n} = \widehat{\bm{a}} = \frac{1}{d}\thinspace\bm{a} =
            /// \frac{\bm{a}}{|\bm{a}|}
            /// $$
            #[must_use]
            pub const fn try_normalize(self) -> Option<Self> {
                let mag = self.magnitude();
                is! { mag == 0.0 || !mag.is_finite(), None, Some(self.div_scalar(mag)) }
            }

            /// Calculates the magnitude of the vector.
            ///
            /// # Formula
            /// $$ \large |\vec{V}| = \sqrt{V_0^2 + ... + V_n^2} $$
            #[must_use]
            pub const fn magnitude(&self) -> $f {
                let mut scale = 0.0;
                let mut has_infinite = false;
                whilst! {  i in 0..D; {
                    let component = Float(self.coords[i]);
                    if component.0.is_nan() { return <$f>::NAN; }
                    let absolute = component.abs().0;
                    if absolute.is_infinite() { has_infinite = true; }
                    else if absolute > scale { scale = absolute; }
                }}
                if has_infinite { return <$f>::INFINITY; }
                if scale == 0.0 { return 0.0; }
                let mut sum = 0.0;
                whilst! {  i in 0..D; {
                    let normalized = self.coords[i] / scale;
                    sum += normalized * normalized;
                }}
                scale * Float(sum).sqrt_hybrid().0
            }

            /// Calculates the squared magnitude of the vector.
            ///
            /// This avoids calculating a square root and can be useful for comparisons,
            /// provided that the squared components and their sum remain representable.
            ///
            /// # Formula
            /// $$ \large |\vec{V}|^2 = V_0^2 + ... + V_n^2 $$
            #[must_use]
            pub const fn magnitude_sq(&self) -> $f { self.dot(self) }

            /// Calculates the magnitude without intermediate scaling.
            ///
            /// This may overflow or underflow while accumulating the squared components.
            #[must_use]
            pub const fn magnitude_unscaled(&self) -> $f {
                Float(self.dot(self)).sqrt_hybrid().0
            }
        }
    };
}
_impl_vector!();
