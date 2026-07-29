// devela/src/num/alg/matrix/primitive.rs
//
//! Const arithmetic for matrices of primitive numeric elements.
//

use crate::{Matrix, NumConst, Vector, unwrap, whilst};

/// Implements const matrix operations for primitive numeric elements.
macro_rules! _impl_matrix {
    () => {
        _impl_matrix![sint i8, i16, i32, i64, i128, isize];
        _impl_matrix![uint u8, u16, u32, u64, u128, usize];
        _impl_matrix![float f32, f64];
    };

    /* operations common to all numeric primitives */

    (@common $t:ty) => {
        impl<const R: usize, const C: usize, const LEN: usize> Matrix<$t, R, C, LEN> {
            /// A matrix whose elements are all zero.
            pub const ZERO: Self = Self::new([<$t>::NUM_ZERO.unwrap(); LEN]);

            /// Adds another matrix entry by entry.
            ///
            /// Each entry follows the arithmetic semantics of its primitive type.
            #[allow(clippy::should_implement_trait)]
            pub const fn add(mut self, other: Self) -> Self {
                whilst! { i in 0..LEN; {
                    self.data[i] = self.data[i] + other.data[i];
                }}
                self
            }
            /// Subtracts another matrix entry by entry.
            ///
            /// Each entry follows the arithmetic semantics of its primitive type.
            #[allow(clippy::should_implement_trait)]
            pub const fn sub(mut self, other: Self) -> Self {
                whilst! { i in 0..LEN; {
                    self.data[i] = self.data[i] - other.data[i];
                }}
                self
            }
            /// Multiplies every matrix entry by `scalar`.
            pub const fn mul_scalar(mut self, scalar: $t) -> Self {
                whilst! { i in 0..LEN; {
                    self.data[i] = self.data[i] * scalar;
                }}
                self
            }
            /// Multiplies this matrix by a column vector.
            ///
            /// An `R × C` matrix maps a `C`-dimensional vector to an
            /// `R`-dimensional vector.
            ///
            /// Each accumulation follows the arithmetic semantics of the
            /// primitive element type.
            pub const fn mul_vector(&self, vector: &Vector<$t, C>) -> Vector<$t, R> {
                let mut result = Vector::new([<$t>::NUM_ZERO.unwrap(); R]);
                whilst! { row in 0..R; {
                    let mut sum = <$t>::NUM_ZERO.unwrap();
                    whilst! { col in 0..C; {
                        sum += self.data[row * C + col] * vector.coords[col];
                    }}
                    result.coords[row] = sum;
                }}
                result
            }

            /* matrix product */

            /// Returns the matrix product of `left` and `right`.
            ///
            /// `left` has shape `R × INNER`, `right` has shape `INNER × C`,
            /// and the resulting matrix has shape `R × C`.
            ///
            /// Each multiplication and accumulation follows
            /// the arithmetic semantics of the primitive element type.
            ///
            /// # Panics
            ///
            /// Panics if the declared backing lengths do not match their matrix dimensions.
            #[must_use]
            pub const fn product<
                const INNER: usize,
                const LEFT_LEN: usize,
                const RIGHT_LEN: usize,
            >(
                left: &Matrix<$t, R, INNER, LEFT_LEN>,
                right: &Matrix<$t, INNER, C, RIGHT_LEN>,
            ) -> Self {
                let mut result = Self::ZERO;

                // Iterate in row-inner-column order. For row-major storage this reuses
                // each left entry while traversing one contiguous row of `right`.
                whilst! { row in 0..R; {
                    let left_row = row * INNER;
                    let result_row = row * C;
                    whilst! { inner in 0..INNER; {
                        let left_value = left.data[left_row + inner];
                        let right_row = inner * C;
                        whilst! { col in 0..C; {
                            result.data[result_row + col] +=
                                left_value * right.data[right_row + col];
                        }}
                    }}
                }}
                result
            }
        }

        impl<const N: usize, const LEN: usize> Matrix<$t, N, N, LEN> {
            /// The square identity matrix.
            pub const IDENTITY: Self = {
                // ZERO validates LEN == N * N before diagonal indexing occurs.
                let mut result = Self::ZERO;
                whilst! { i in 0..N; {
                    result.data[i * N + i] = <$t>::NUM_ONE.unwrap();
                }}
                result
            };
            /// Multiplies this square matrix by another square matrix.
            #[must_use]
            pub const fn mul_square(&self, other: &Self) -> Self {
                Self::product(self, other)
            }
            /// Returns the trace: the sum of the main diagonal entries.
            #[must_use]
            pub const fn trace(&self) -> $t {
                let mut result = <$t>::NUM_ZERO.unwrap();
                whilst! { i in 0..N; {
                    result += self.data[i * N + i];
                }}
                result
            }
        }
    };

    /* integer operations */

    (int $($t:ty),+ $(,)?) => {
        $( _impl_matrix![@int $t]; )+
    };
    (@int $t:ty) => {
        _impl_matrix![@common $t];

        impl<const R: usize, const C: usize, const LEN: usize> Matrix<$t, R, C, LEN> {
            /// Divides every matrix entry by `scalar`.
            ///
            /// # Panics
            ///
            /// Panics if `scalar` is zero.
            ///
            /// For signed integers, it also panics if an entry is the minimum
            /// representable value and `scalar` is `-1`.
            pub const fn div_scalar(mut self, scalar: $t) -> Self {
                assert!(scalar != 0, "attempt to divide a matrix by zero");
                whilst! { i in 0..LEN; {
                    self.data[i] = self.data[i] / scalar;
                }}
                self
            }
            /// Returns the entry-wise quotient.
            ///
            /// Returns `None` if `scalar` is zero or if any entry division
            /// overflows.
            #[must_use]
            pub const fn checked_div_scalar(mut self, scalar: $t) -> Option<Self> {
                if scalar == 0 { return None; }
                whilst! { i in 0..LEN; {
                    self.data[i] = unwrap![some? self.data[i].checked_div(scalar)];
                }}
                Some(self)
            }

            /* matrix product */

            /// Returns the checked matrix product of `left` and `right`.
            ///
            /// Returns `None` if any individual multiplication or accumulation
            /// overflows.
            ///
            /// `left` has shape `R × INNER`, `right` has shape `INNER × C`,
            /// and the resulting matrix has shape `R × C`.
            #[must_use]
            pub const fn checked_product<
                const INNER: usize,
                const LEFT_LEN: usize,
                const RIGHT_LEN: usize,
            >(
                left: &Matrix<$t, R, INNER, LEFT_LEN>,
                right: &Matrix<$t, INNER, C, RIGHT_LEN>,
            ) -> Option<Self> {
                let mut result = Self::ZERO;
                whilst! { row in 0..R; {
                    let left_row = row * INNER;
                    let result_row = row * C;
                    whilst! { inner in 0..INNER; {
                        let left_value = left.data[left_row + inner];
                        let right_row = inner * C;
                        whilst! { col in 0..C; {
                            let product = unwrap![some?
                                left_value.checked_mul(right.data[right_row + col])
                            ];
                            let result_index = result_row + col;
                            result.data[result_index] = unwrap![some?
                                result.data[result_index].checked_add(product)
                            ];
                        }}
                    }}
                }}
                Some(result)
            }
        }

        impl<const N: usize, const LEN: usize> Matrix<$t, N, N, LEN> {
            /// Returns the checked product of two square matrices.
            ///
            /// Returns `None` if any multiplication or accumulation overflows.
            #[must_use]
            pub const fn checked_mul_square(&self, other: &Self) -> Option<Self> {
                Self::checked_product(self, other)
            }
        }
    };

    /* signed integer operations */

    (sint $($t:ty),+ $(,)?) => {
        $( _impl_matrix![@sint $t]; )+
    };

    (@sint $t:ty) => {
        _impl_matrix![int $t];

        impl<const R: usize, const C: usize, const LEN: usize> Matrix<$t, R, C, LEN> {
            /// Negates every matrix entry.
            ///
            /// Each entry follows the negation semantics of its primitive type.
            pub const fn neg(mut self) -> Self {
                whilst! { i in 0..LEN; {
                    self.data[i] = -self.data[i];
                }}
                self
            }
            /// Returns the entry-wise negation.
            ///
            /// Returns `None` if any entry is the minimum representable value.
            #[must_use]
            pub const fn checked_neg(mut self) -> Option<Self> {
                whilst! { i in 0..LEN; {
                    self.data[i] = unwrap![some? self.data[i].checked_neg()];
                }}
                Some(self)
            }
        }
    };

    /* unsigned integer operations */

    (uint $($t:ty),+ $(,)?) => {
        $( _impl_matrix![@uint $t]; )+
    };
    (@uint $t:ty) => {
        _impl_matrix![int $t];
    };

    /* floating-point operations */

    (float $($t:ty),+ $(,)?) => {
        $( _impl_matrix![@float $t]; )+
    };
    (@float $t:ty) => {
        _impl_matrix![@common $t];

        impl<const R: usize, const C: usize, const LEN: usize> Matrix<$t, R, C, LEN> {
            /// Negates every matrix entry.
            pub const fn neg(mut self) -> Self {
                whilst! { i in 0..LEN; {
                    self.data[i] = -self.data[i];
                }}
                self
            }
            /// Divides every matrix entry by `scalar`.
            ///
            /// Each entry follows IEEE 754 floating-point division semantics.
            pub const fn div_scalar(mut self, scalar: $t) -> Self {
                whilst! { i in 0..LEN; {
                    self.data[i] = self.data[i] / scalar;
                }}
                self
            }
        }
    };
}
_impl_matrix!();
