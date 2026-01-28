// devela::num::absence
//
//! Allows the unit type [`()`] to represent the absence of a number.
//

use crate::NoNum;

#[rustfmt::skip]
impl super::Num for NoNum {
    type Inner = ();
    type Out = ();
    type Rhs = ();

    fn num_into(self) -> Self::Inner {}
}

#[cfg(feature = "int")]
impl super::NumInt for NoNum {
    type OutI = ();
}

// #[cfg(feature = "geom")]
// mod geom {
//     use crate::{NoNum, NumVector};
//
//     #[cfg_attr(nightly_doc, doc(cfg(feature = "geom")))]
//     impl NumVector for NoNum {
//         type Scalar = ();
//     }
// }
