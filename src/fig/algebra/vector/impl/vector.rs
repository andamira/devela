// devela::fig::algebra::vector::impl::vector
//
//! impl methods for Vector
//

#![allow(clippy::needless_range_loop)]

use super::super::Vector;

/* common methods */

impl<T, const D: usize> Vector<T, D> {
    /// Returns a new `Vector` from the given `array`.
    pub const fn new(array: [T; D]) -> Self {
        Self { array }
    }
}

/* compile-time ops for primitives */

// helper for methods
macro_rules! impl_vector {
    (int $($t:ty),+) => { $( impl_vector![@int $t]; )+ };
    (@int $t:ty) => {
        impl<const D: usize> Vector<$t, D> {
            /// Adds two vectors together, in compile-time.
            pub const fn c_add(&self, other: Self) -> Self {
                let mut result = [0; D];
                let mut i = 0;
                while i < D {
                    result[i] = self.array[i] + other.array[i];
                    i += 1;
                }
                Vector::new(result)
            }
        }
    };
    (float $($t:ty),+) => { $( impl_vector![@float $t]; )+ };
    (@float $t:ty) => {
        impl<const D: usize> Vector<$t, D> {
            /// Adds two vectors together.
            pub fn add(&self, other: Self) -> Self {
                let mut result = [0.0; D];
                let mut i = 0;
                while i < D {
                    result[i] = self.array[i] + other.array[i];
                    i += 1;
                }
                Vector::new(result)
            }
        }
    };
}
impl_vector![int u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
impl_vector![float f32, f64];
