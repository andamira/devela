// devela_base_alloc::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()] // public
#![doc = crate::_doc!(modules: crate::sys; mem)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc, borrow, boxed, cell, mem, pin, ptr, rc, slice)]
//

mod _reexport; // SYMLINK from /src/sys/mem/_reexport_alloc.rs

mod alloc;
mod borrow;
// mod pin;
// mod ptr;

// pub mod cell;

crate::structural_mods! { // _mods, _pub_mods, _reexports
    _mods {
        pub use super::{
            alloc::_all::*,
            borrow::_all::*,
            // pin::_all::*,
            // ptr::_all::*,
        };
    }
    _pub_mods {
        // pub use super::cell::_all::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
