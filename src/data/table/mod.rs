// devela::data::table
//
//! Tabular and heterogeneous data processing.
//!
#![doc = crate::_doc!(modules: crate::data; table)]
#![doc = crate::_doc!(flat:"data")]
//

pub mod value;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::value::_all::*;
    }
    _crate_internals {
        pub(crate) use super::value::_crate_internals::*;
    }
}
