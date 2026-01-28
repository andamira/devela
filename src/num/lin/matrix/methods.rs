// devela::num::lin::matrix::methods
//
//! Methods for matrices.
//
// TODO
// - rank
// - is_square
// - is ... (see paper)
// - impl ops

use crate::{Matrix, NumConst, is};

impl<const R: usize, const C: usize, const LEN: usize, const MAX_LEN_DET: usize, T: Copy>
    Matrix<T, R, C, LEN, true, MAX_LEN_DET>
{
    /// Returns the transposed matrix (C × R).
    pub const fn transpose(&self) -> Matrix<T, C, R, LEN, true, MAX_LEN_DET> {
        let mut transposed_data = [self.data[0]; LEN];
        let mut i = 0;
        while i < R {
            let mut j = 0;
            while j < C {
                transposed_data[j * R + i] = self.data[i * C + j];
                j += 1;
            }
            i += 1;
        }
        Matrix { data: transposed_data }
    }
}

/* numeric methods */

macro_rules! impl_matrix {
    () => {
        impl_matrix![int: i8, i16, i32, i64, i128, isize];
        impl_matrix![float: f32, f64];
    };
    (float: $($T:ty),+) => { $(
        impl_matrix![@both: $T];
        // impl_matrix![@float: $T];
    )+ };
    (int: $($T:ty),+) => { $(
        impl_matrix![@both: $T];
        // impl_matrix![@int: $T];
    )+ };

    (@both: $T:ty) => {
        // methods for row-major matrices
        impl<const R: usize, const C: usize, const LEN: usize, const MAX_LEN_DET: usize>
            Matrix<$T, R, C, LEN, true, MAX_LEN_DET> {

            /// Returns a new matrix with the given `data`.
            pub const fn new(data: [$T; LEN]) -> Self {
                Self { data }
            }

            /* misc */

            /// Returns the identity matrix, or `None` if the matrix is not square.
            pub const fn identity() -> Option<Self> {
                if R == C { Some(Self::identity_unchecked()) } else { None }
            }

            /// Creates an NxN identity matrix.
            ///
            /// # Panics
            /// Panics if the matrix is not square.
            pub const fn identity_unchecked() -> Self {
                let data = [<$T>::NUM_ZERO.unwrap(); LEN];
                let mut matrix = Matrix { data };
                let mut i = 0;
                while i < R {
                    *matrix.at_mut(i, i) = <$T>::NUM_ONE.unwrap();
                    i += 1;
                }
                matrix
            }

            /* ops */

            /// Returns the element-wise sum of two matrices.
            pub const fn add(&self, other: &Self) -> Self {
                let mut result = [self.data[0]; LEN];
                let mut i = 0;
                while i < LEN {
                    result[i] = self.data[i] + other.data[i];
                    i += 1;
                }
                Self { data: result }
            }

            /// Returns the matrix scaled by a scalar value.
            pub const fn scale(&self, scalar: $T) -> Self {
                let mut result = [self.data[0]; LEN];
                let mut i = 0;
                while i < LEN {
                    result[i] = self.data[i] * scalar;
                    i += 1;
                }
                Self { data: result }
            }

            /// Computes the matrix product (R × C) * (C × C2) = (R × C2) = LEN2.
            pub const fn mul<const C2: usize, const LEN2: usize>(
                &self, other: &Matrix<$T, C, C2, LEN2, true, MAX_LEN_DET>,
            ) -> Matrix<$T, R, C2, LEN2, true, MAX_LEN_DET> {
                let mut result = [<$T>::NUM_ZERO.unwrap(); LEN2];
                let mut i = 0;
                while i < R {
                    let mut j = 0;
                    while j < C2 {
                        let mut sum = self.data[i * C] * other.data[j]; // 1st element mul
                        let mut k = 1;
                        while k < C {
                            sum += self.data[i * C + k] * other.data[k * C2 + j];
                            k += 1;
                        }
                        result[i * C2 + j] = sum;
                        j += 1;
                    }
                    i += 1;
                }
                Matrix { data: result }
            }

            /* determinant */

            /// Returns the determinant if the matrix is square, or `None` otherwise.
            ///
            /// # Panics
            /// - If called on a non-square matrix in debug mode, it will panic.
            pub const fn determinant(&self) -> Option<$T> {
                is![R == C; Some(self.determinant_unchecked()); None]
            }

            /// Returns the determinant without checking matrix squareness.
            ///
            /// # Panics
            /// - Panics in debug mode if `R != C` (non-square matrix).
            /// - May panic on overflow for integer types.
            pub const fn determinant_unchecked(&self) -> $T {
                debug_assert![R == C, "a non-square matrix has no determinant"];
                let s = self;
                match R {
                    1 => s.at(0, 0),
                    2 => s.at(0, 0) * s.at(1, 1) - s.at(0, 1) * s.at(1, 0),
                    3 => { s.at(0, 0) * (s.at(1, 1) * s.at(2, 2) - s.at(1, 2) * s.at(2, 1))
                         - s.at(0, 1) * (s.at(1, 0) * s.at(2, 2) - s.at(1, 2) * s.at(2, 0))
                         + s.at(0, 2) * (s.at(1, 0) * s.at(2, 1) - s.at(1, 1) * s.at(2, 0)) }
                    _ => {
                        debug_assert![R*C <= MAX_LEN_DET, "MAX_LEN_DET is too small"];
                        let mut buffer = [<$T>::NUM_ZERO.unwrap(); MAX_LEN_DET];
                        Self::submatrix_determinant(R, &s.data, &mut buffer)
                    }
                }
            }

            /// Extracts a `(D-1)x(D-1)` submatrix by removing the given row and column.
            ///
            /// Arguments
            /// - `n`: Matrix dimension.
            /// - `row`, `col`: The row and column to exclude.
            /// - `matrix`: Source matrix in row-major order.
            /// - `buffer`: Target buffer to store the submatrix.
            ///
            /// # Panics
            /// Panics in debug mode if `buffer.len() < (n-1)*(n-1)`.
            pub const fn submatrix(
                n: usize,
                row: usize,
                col: usize,
                matrix: &[$T],
                buffer: &mut [$T]
            ) {
                debug_assert!(buffer.len() >= (n-1)*(n-1), "Buffer is too small");
                let (mut idx, mut r) = (0, 0);
                while r < n {
                    is![r == row; { r += 1; continue; }];
                    let mut c = 0;
                    while c < n {
                        is![c == col; { c += 1; continue; }];
                        buffer[idx] = matrix[r * n + c];
                        idx += 1;
                        c += 1;
                    }
                    r += 1;
                }
            }

            /// Computes the determinant of a square matrix using Laplace expansion.
            ///
            /// This method is **only valid for square matrices** of size `dim × dim`
            /// and is intended for cases where `dim > 3`.
            ///
            /// How it works:
            /// - Expands along the first row using minors (submatrices).
            /// - Recursively computes determinants of `(dim-1)×(dim-1)` matrices.
            ///
            /// Notes:
            /// - Uses a temporary `buffer` to avoid repeated allocations.
            /// - The `buffer` must be at least `(dim-1)^2 + (dim-2)^2` elements long.
            /// - `MAX_LEN_DET` defines the upper bound for buffer size.
            /// - It has `O(N!) factorial time complexity due to recursive expansion.
            ///
            /// # Panics
            /// - Panics in debug mode if `buffer.len()` is insufficient.
            /// - Panics if matrix is not square (should never happen when used internally).
            pub const fn submatrix_determinant(dim: usize, matrix: &[$T], buffer: &mut [$T]) -> $T {
                is![dim == 1; return matrix[0]];
                is![dim == 2; return matrix[0] * matrix[3] - matrix[1] * matrix[2]];

                let required_size = (dim - 1) * (dim - 1) + (dim - 2) * (dim - 2);
                debug_assert!(buffer.len() >= required_size, "buffer is too small");

                let (mut determinant, mut col_idx) = (<$T>::NUM_ZERO.unwrap(), 0);
                while col_idx < dim {
                    let minor_len = (dim - 1) * (dim - 1);
                    // 1. Split buffer into current level & next (for recursion)
                    let (minor_matrix, next_minor_buffer) = buffer.split_at_mut(minor_len);
                    // 2. Extract the minor submatrix
                    Self::submatrix(dim, 0, col_idx, matrix, minor_matrix);
                    // 3. Compute determinant recursively
                    let sub_det = Self::submatrix_determinant(dim - 1, minor_matrix, next_minor_buffer);
                    determinant += Self::parity_sign(col_idx) * matrix[col_idx] * sub_det;
                    col_idx += 1;
                }
                determinant
            }

            /* utility methods */

            /// Returns an immutable reference to the element at (`row`, `col`).
            #[inline(always)]
            pub const fn at(&self, row: usize, col: usize) -> $T {
                self.data[row * C + col]
            }
            /// Returns a shared reference to the element at (`row`, `col`).
            #[inline(always)]
            pub const fn at_ref(&self, row: usize, col: usize) -> &$T {
                &self.data[row * C + col]
            }
            /// Returns an exclusive reference to the element at (`row`, `col`).
            #[inline(always)]
            pub const fn at_mut(&mut self, row: usize, col: usize) -> &mut $T {
                &mut self.data[row * C + col]
            }

            /// Returns alternating ±1 based on the column index for determinant expansion.
            ///
            /// Returns `1` for even indices, and `-1` for odd indices.
            #[inline(always)]
            const fn parity_sign(i: usize) -> $T {
                is![i % 2 == 0; <$T>::NUM_ONE.unwrap(); <$T>::NUM_NEG_ONE.unwrap()]
            }
        }
    };
    // (@float: $T:ty) => {};
    // (@int: $T:ty) => {}
}
impl_matrix![];
