// devela_base_core::sys::mem::bound
//
#![doc = crate::_DOC_SYS_MEM_BOUND!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; bound)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: pin, ptr)]
//

mod align; // CacheAlign, MemAligned
mod cswap; // cswap!
mod pin;
mod ptr;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            align::_all::*,
            cswap::*,
            pin::_all::*,
            ptr::_all::*,
        };
    }
}
