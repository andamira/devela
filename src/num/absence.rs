// devela::num::absence
//
//! Allows the unit type [`()`] to represent the absence of a number.
//

#[doc = crate::_TAG_NO!()]
#[doc = crate::_TAG_NUM!()]
/// Represents the absence of a number.
///
/// This can be used anywhere an implementation of [`Num`][super::Num] is expected,
/// since it implements all the numeric traits, but does nothing.
pub type NoNum = ();

#[rustfmt::skip]
impl super::Num for NoNum {
    type Inner = ();
    type Out = ();
    type Rhs = ();

    fn num_into(self) -> Self::Inner {}
}

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
