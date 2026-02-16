// devela::data::value
//
#![doc = crate::_DOC_DATA_VALUE!()] // public
#![doc = crate::_doc!(modules: crate::data; value: tuple)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod macros; // internal macros

mod build;
mod traits; // DataValue(Copy), DataType(Copy), DataRaw(Copy)

mod of; // Oneof

#[cfg(feature = "_tuple")]
pub mod tuple; // Tuple, TupleFmt, TupleEnumRef, TupleEnumMut

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            build::*,
            traits::*,

            of::_all::*,
        };
    }
    _pub_mods {
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
    }
    _crate_internals {
        pub(crate) use super::macros::_crate_internals::*;
    }
}
