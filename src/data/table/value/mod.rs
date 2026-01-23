// devela::data::table::value
//
#![doc = crate::_TAG_WIP!()]
//! Enumerated data values and types, classified by size.
//

mod macros; // internal macros

mod build;
mod traits; // DataValue(Copy), DataType(Copy), DataRaw(Copy)

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{build::*, traits::*};
    }
    _crate_internals {
        pub(crate) use super::macros::_crate_internals::*;
    }
}
