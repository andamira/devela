// devela_base_std::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()]
//

mod _reexport; // SYMLINK from /src/work/sync/_reexport_std.rs

pub mod mpsc;

crate::structural_mods! { // _pub_mods, _reexport
    _pub_mods {
        pub use super::{
            mpsc::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
