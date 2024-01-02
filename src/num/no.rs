// devela::num::no
//
//! Not a number.
//

use super::{Num, NumInt};

/// Represents the absence of a number.
///
/// This can be used anywhere an implementation of [`Num`] is expected,
/// since it implements all the numeric traits, but does nothing.
pub type NoNum = ();

#[rustfmt::skip]
impl Num for NoNum {
    type Inner = ();
    type Out = ();
    type Rhs = ();

    fn num_into(self) -> Self::Inner {}
}

impl NumInt for NoNum {}
