//
#![doc = crate::_DOC_CODE_OPS_RANGE!()] // public
#![doc = crate::_doc!(modules: crate::code::ops; range)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: range)]
//

crate::mods_in! {}
crate::mods_out! { // _reexports
    _reexports {
        pub use crate::boot::rust::code::ops_range::*;
    }
}
