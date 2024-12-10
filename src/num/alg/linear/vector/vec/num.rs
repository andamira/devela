// devela::num::alg::linear::vector::vec::num
//
//!
//

use crate::{Num, NumVector, Vec, VecVector};

impl<T: Num> NumVector for VecVector<T> {
    type Scalar = T;
}

impl<T: Num> Num for VecVector<T> {
    type Inner = Vec<T>;
    type Out = Self;
    type Rhs = Self;

    fn num_into(self) -> Self::Inner {
        self.coords
    }
}
