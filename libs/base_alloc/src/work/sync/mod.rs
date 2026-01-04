// devela_base_alloc::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()]
//!
#![doc = crate::_doc!(extends: sync)]
//
// safety
#![cfg_attr(base_safe_work, forbid(unsafe_code))]

mod _reexport;

crate::structural_mods! { // _reexports
    _reexports { pub use super::_reexport::*; }
}
