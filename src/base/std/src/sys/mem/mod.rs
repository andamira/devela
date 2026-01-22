// devela_base_std::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()]
//

mod alloc;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            alloc::_all::*,
        };
    }
}
