// devela::num::geom::algebra::vector::impl_vec::num
//
//!
//

use super::super::{NumVector, VecVector};
use crate::data::Vec;
use crate::num::Num;

impl<T: Num> NumVector for VecVector<T> {
    type Scalar = T;
}

impl<T: Num> Num for VecVector<T> {
    type Inner = Vec<T>;
    type Out = Self;
    type Rhs = Self;

    #[inline]
    fn num_into(self) -> Self::Inner {
        self.vec
    }
}
