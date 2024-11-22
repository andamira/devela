// devela::num::algebra::linear::vector::array::num
//
//!
//
// IMPROVE

use super::super::{NumVector, Vector};
use crate::num::Num;

impl<T: Num, const D: usize> NumVector for Vector<T, D> {
    type Scalar = T;
}

impl<T: Num, const D: usize> Num for Vector<T, D> {
    type Inner = [T; D];
    type Out = Self;
    type Rhs = Self;

    fn num_into(self) -> Self::Inner {
        self.coords
    }
}
