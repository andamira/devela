// devela_base_std::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()]
//

mod reexports;

pub mod mpsc;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            reexports::*,
        };
    }
    _pub_mods {
        pub use super::{
            mpsc::_all::*,
        };
    }
}
