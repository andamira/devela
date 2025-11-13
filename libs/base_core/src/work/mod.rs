// devela_base_core::work
//
#![doc = crate::_DOC_WORK!()]
#![doc = crate::_doc!(modules: crate; work: future, process, sync, thread)]
#![doc = "<br/><hr>"] // gives way to zall
//!
#![doc = crate::_doc!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(base_safe_work, forbid(unsafe_code))]

pub mod future;
pub mod sync;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            future::_all::*,
            sync::_all::*,
        };
    }
}
