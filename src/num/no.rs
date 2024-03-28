// devela::num::no
//
//! Not a number.
//

/// Represents the absence of a number.
///
/// This can be used anywhere an implementation of [`Num`] is expected,
/// since it implements all the numeric traits, but does nothing.
pub type NoNum = ();

/* num */

#[rustfmt::skip]
impl super::Num for NoNum {
    type Inner = ();
    type Out = ();
    type Rhs = ();

    fn num_into(self) -> Self::Inner {}
}

#[cfg(feature = "num_int")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_int")))]
impl super::NumInt for NoNum {
    type OutI = ();
}
