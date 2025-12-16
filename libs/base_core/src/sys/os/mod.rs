// devela_base_core::sys::os
//
#![doc = crate::_DOC_SYS_OS!()]
#![doc = crate::_doc!(modules: crate::sys; os: fd)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: os)]
//

pub mod fd; // FdRaw

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            fd::_all::*,
        };
    }
}
