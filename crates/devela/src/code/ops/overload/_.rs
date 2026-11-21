//
#![doc = crate::_DOC_CODE_OPS_OVERLOAD!()] // public
#![doc = crate::_doc!(modules: crate::code::ops; overload)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: ops)]
//

crate::mods_in! {}
crate::mods_out! { // _reexports
    _reexports {
        pub use crate::boot::rust::code::ops_overload::*;
    }
}
