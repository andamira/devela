// devela/src/sys/mem/bound/_.rs
//
#![doc = crate::_DOC_SYS_MEM_BOUND!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; bound)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: pin, ptr)]
//

crate::mods_in! {
    mod_ align; // CacheAlign, MemAligned
    mod cswap; // cswap!
    mod_ pin; // Pinned, ::core::pin::*
    mod_ ptr; // FatPtr, Ptr, ::core::ptr::*
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            align::_all::*,
            cswap::cswap,
            pin::_all::*,
            ptr::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            align::{CacheAlign, MemAligned},
            cswap::cswap,
        };
    }
}
