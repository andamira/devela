// devela::fig::algebra::vector::num_vector
//
//!
//

use super::super::{NumVector, Vector};
use crate::num::{Num, NumResult as Result};

/* impl for Vector */

impl<T: Num, const D: usize> NumVector for Vector<T, D> {
    type Scalar = T;
}

impl<T: Num, const D: usize> Num for Vector<T, D> {
    type Inner = [T; D];
    type Out = Self;
    type Rhs = Self;

    #[inline]
    fn num_into(self) -> Self::Inner {
        self.array
    }

    // #[inline]
    // fn num_from(from: Self::Inner) -> Result<Self> { Ok(from) }
    // #[inline]
    // fn num_from_ref(from: &Self::Inner) -> Result<Self> { Ok(*from) }
    // #[inline]
    // fn num_set(&mut self, value: Self::Inner) -> Result<()> { *self = value; Ok(()) }
    // #[inline]
    // fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> {
    //     *self = *value; Ok(())
    // }
}

/* impl for VecVector */

#[cfg(feature = "alloc")]
mod alloc {
    use super::super::super::{NumVector, VecVector};
    use super::{Num, Result};
    use crate::data::Vec;

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
}
