// devela::data::value
//
#![doc = crate::_DOC_DATA_VALUE!()] // public
#![doc = crate::_doc!(modules: crate::data; value: tuple)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod absence; // NoData
mod of; // Oneof

// mod decode; // ValueDecode WIP
mod kind; // ValueKind, ValueKind4, WIP ValueKindSet
// mod schema; // WIP Schemas for encoded data structures.
// mod value; // value! WIP
#[cfg(feature = "_tuple")]
pub mod tuple; // Tuple, TupleFmt, TupleEnumRef, TupleEnumMut

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            absence::*,
            // decode::*,
            of::_all::*,
            kind::*,
            // schema::_all::*,
            // value::*,
        };
    }
    _pub_mods {
        #[cfg(feature = "_tuple")]
        pub use super::tuple::_all::*;
    }
}
