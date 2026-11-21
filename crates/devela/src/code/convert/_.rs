//
#![doc = crate::_DOC_CODE_CONVERT!()] // public
#![doc = crate::_doc!(modules: crate::code; convert)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: convert)]
//

crate::mods_in! {}
crate::mods_out! { // _reexports
    _reexports {
        pub use crate::boot::rust::code::convert::*;
    }
}
