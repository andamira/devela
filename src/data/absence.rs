// devela::data::absence
//
//! Implements traits for the unit type [`()`] to represent the absence of data.
//

#[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
use super::DataRaw;
use crate::{DataType, DataTypeCopy, DataValue, DataValueCopy};

#[doc = crate::_TAG_NO!()]
#[doc = crate::_TAG_DATA!()]
/// Represents the absence of some data.
#[doc = crate::_doc!(location: "data")]
pub type NoData = ();

#[rustfmt::skip]
impl DataType for NoData {
    type Value = NoData;

    fn data_values_are_copy() -> bool { true }

    fn data_value_is_copy(&self) -> bool { true }
    fn data_value_default(&self) -> Option<Self::Value> { Some(()) }
    fn data_value_align(&self) -> usize { align_of::<NoData>() }
    fn data_value_size(&self) -> usize { size_of::<NoData>() }
}
#[rustfmt::skip]
impl DataValue for NoData {
    type Type = NoData;

    fn data_values_are_copy() -> bool { true }

    fn data_type(&self) -> Self::Type {}
    fn data_value_is_copy(&self) -> bool { true }
}

impl DataTypeCopy for NoData {}
impl DataValueCopy for NoData {}

#[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
unsafe impl DataRaw for NoData {}
