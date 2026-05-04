// devela::data::value
//
#![doc = crate::_DOC_DATA_VALUE!()] // public
#![doc = crate::_doc!(modules: crate::data; value: tuple)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod absence; // NoData
mod of; // Oneof

#[cfg(feature = "_tuple")]
pub mod tuple; // Tuple, TupleFmt, TupleEnumRef, TupleEnumMut

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            absence::*,
            of::_all::*,
        };
    }
    _pub_mods {
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
    }
}
