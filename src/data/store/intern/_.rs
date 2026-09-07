// devela/src/data/store/intern/_.rs
//
#![doc = crate::_DOC_DATA_STORE_INTERN!()] // public
#![doc = crate::_doc!(modules: crate::data::store; intern)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Interners retain one canonical representative for each equal value.
//!
//! [`intern_string!`] specializes this model for UTF-8 strings,
//! storing each distinct string once and returning compact symbols
//! that resolve to its canonical copy.
//

crate::mods_in! {
    mod_ string; // intern_string!
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            string::_all::*,
        };
    }
    _hidden {
        pub use super::{
            string::_hidden::*,
        };
    }
}
