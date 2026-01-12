// devela::geom::linear::vector::vec::methods
//
//! impl methods for VecVector
//

#![allow(clippy::needless_range_loop)]

use super::super::VecVector;
use crate::data::Vec;

/* common methods */

impl<T> VecVector<T> {
    /// Returns a new `VecVector` from the given `coords` vector.
    pub const fn new(coords: Vec<T>) -> Self {
        Self { coords }
    }
}

/* compile-time ops for primitives */

// helper for methods
macro_rules! impl_vector {
    (int $($t:ty),+) => { $( impl_vector![@int $t]; )+ };
    (@int $t:ty) => {
        impl VecVector<$t> {
        }
    };
    (float $($t:ty),+) => { $( impl_vector![@float $t]; )+ };
    (@float $t:ty) => {
        impl VecVector<$t> {
        }
    };
}
impl_vector![int u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
impl_vector![float f32, f64];
