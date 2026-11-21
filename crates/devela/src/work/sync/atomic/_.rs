//
#![doc = crate::_DOC_WORK_SYNC_ATOMIC!()] // public
#![doc = crate::_doc!(modules: crate::work::sync; atomic)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//

crate::mods_in! {}
crate::mods_out! { // _reexports
    _reexports {
        pub use crate::boot::rust::work::sync_atomic::*;
    }
}
