// devela::data::value
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_DATA_VALUE!()] // public
#![doc = crate::_doc!(modules: crate::data; value)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
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
