// devela_base_core::sys::mem::ptr
//
#![doc = crate::_DOC_SYS_MEM_PTR!()]
//

mod namespace; // Ptr
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::Ptr, reexports::*,
        };
    }
}
