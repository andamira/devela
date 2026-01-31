// devela_base_std::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()] // public
#![doc = crate::_doc!(modules: crate::sys; mem)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
//

mod alloc;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            alloc::_all::*,
        };
    }
}
