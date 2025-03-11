// devela::num::no
//
//! Allows the unit type [`()`] to represent the absence of a number.
//

#[doc = crate::TAG_NO!()]
#[doc = crate::TAG_NUM!()]
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

#[cfg(_int··)]
#[cfg_attr(nightly_doc, doc(cfg(_int··)))]
impl super::NumInt for NoNum {
    type OutI = ();
}

// #[cfg(feature = "geom")]
// mod geom {
//     use crate::{NoNum, NumVector};
//
//     #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "geom")))]
//     impl NumVector for NoNum {
//         type Scalar = ();
//     }
// }
