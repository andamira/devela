//
#![doc = crate::_DOC_WORK_SYNC_MPSC!()] // public
#![doc = crate::_doc!(modules: crate::work::sync; mpsc)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]

crate::mods_in! {
    mod namespace; // Mpsc
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::namespace::*;
    }
    _reexports {
        pub use crate::boot::rust::work::sync_mpsc::*;
    }
}
