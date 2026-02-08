// devela_base_core::sys::mem::alloc
//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; alloc)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc)]
//

// mod alloc;
mod arena; // define_arena!
// mod storage;

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            arena::_all::*,
        };
    }
    _hidden {
        pub use super::{
            arena::_hidden::*,
        };
    }
}
