// devela_base_core::num::dom::no
//
//! Allows the unit type [`()`] to represent the absence of a number.
//

#[doc = crate::_tags!(no num)]
/// Represents the absence of a number.
#[doc = crate::_doc_location!("num")]
///
/// This can be used anywhere an implementation of [`Num`][super::Num] is expected,
/// since it implements all the numeric traits, but does nothing.
pub type NoNum = ();
