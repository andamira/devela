// devela/src/num/dom/no.rs
//
//! Allows the unit type [`()`] to represent the absence of a number.
//

#[doc = crate::_tags!(no num)]
/// Represents the absence of a number.
#[doc = crate::_doc_meta!{
    location("num", type NoNum),
}]
pub type NoNum = ();

#[cfg(feature = "num")]
impl super::Num for NoNum {
    type Inner = ();
    type Out = ();
    type Rhs = ();

    fn num_into(self) -> Self::Inner {}
}

#[cfg(all(feature = "num", feature = "int"))]
impl super::NumInt for NoNum {
    type OutI = ();
}
