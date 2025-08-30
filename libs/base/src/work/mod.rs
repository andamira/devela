// devela_base::work
//
#![doc = crate::_DOC_WORK!()]
//

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
