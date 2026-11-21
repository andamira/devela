//
#![doc = crate::_DOC_WORK_SYNC_ATOMIC!()] // public
#![doc = crate::_doc!(modules: crate::work::sync; atomic)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//

crate::mods_in! {
    mod _reexport_dep; // from dep_atomic, dep_portable_atomic (and impls ConstInit)
}
crate::mods_out! { // _reexports
    _reexports {
        pub use crate::boot::rust::work::sync_atomic::*;
        pub use super::_reexport_dep::*;
    }
}
