//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; alloc)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc)]
//

crate::mods_in! {
    mod_ alloc; // Alloc, AllocBump, AllocLinuxMmap, AllocWasm…
    mod_ storage; // Bare, BareBox, Boxed, Storage
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            alloc::_all::*,
            storage::_all::*,
        };
    }
}
