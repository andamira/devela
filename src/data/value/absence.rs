// devela/src/data/value/absence.rs
//
//! Implements traits for the unit type [`()`] to represent the absence of data.
//

#[doc = crate::_tags!(no data)]
/// Represents the absence of some data.
#[doc = crate::_doc_meta!{
    location("data/value", type NoData),
    test_size_of(NoData = 0),
}]
pub type NoData = ();
