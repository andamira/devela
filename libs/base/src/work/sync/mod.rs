// devela_base::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()]
//

pub mod atomic;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            atomic::_all::*,
        };
    }
}
