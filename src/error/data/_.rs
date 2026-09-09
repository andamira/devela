// devela/src/error/data/_.rs
//
#![doc = crate::_DOC_ERROR_DATA!()] // public
#![doc = crate::_doc!(modules: crate::error; data)]
#![doc = crate::_doc!(flat:"error")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod capacity;
    mod other;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            capacity::*,
            other::*,
        };
    }
}
