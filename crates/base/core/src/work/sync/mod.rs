// devela_base_core::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()] // public
#![doc = crate::_doc!(modules: crate::work; sync)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//

pub mod atomic;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            atomic::_all::*,
        };
    }
}
