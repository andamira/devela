// devela_base_alloc::work
//
#![doc = crate::_DOC_WORK!()]
#![doc = crate::_doc!(modules: crate; work: sync)] // future, process, thread
#![doc = "<br/><hr>"] // gives way to zall
//!
#![doc = crate::_doc!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(base_safe_work, forbid(unsafe_code))]

pub mod sync;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            sync::_all::*,
        };
    }
}
