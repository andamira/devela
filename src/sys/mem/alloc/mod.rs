// devela/src/sys/mem/alloc/mod.rs
//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; alloc)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc)]
//

mod alloc; // Alloc, BumpAlloc, LinuxMmapAlloc, WasmAlloc, reexports
mod storage; // Bare, BareBox, Boxed, Storage

crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            alloc::_all::*,
            storage::*,
        };
    }
}
