// devela_base_std::sys::os
//
#![doc = crate::_DOC_SYS_OS!()] // public
#![doc = crate::_doc!(modules: crate::sys; os: fd)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: os)]
//

pub mod fd;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::fd::*;
    }
}
