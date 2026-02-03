// devela::sys::mem::alloc
//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; alloc)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc)]
//

mod alloc; // Alloc, ::alloc::alloc::*
// mod arena;
mod storage; // Bare, BareBox, Boxed, Storage

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            alloc::_all::*,
            storage::*,
        };
    }
}
