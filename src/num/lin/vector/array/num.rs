// devela::num::lin::vector::array::num
//
//!
//

use crate::{Num, NumVector, Vector};

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
