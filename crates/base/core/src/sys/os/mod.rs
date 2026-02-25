// devela_base_core::sys::os
//
#![doc = crate::_DOC_SYS_OS!()] // public
#![doc = crate::_doc!(modules: crate::sys; os: fd)] // browser, windows
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: os)]
//

pub mod fd; // FdRaw
pub mod term;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            fd::_all::*,
            term::_all::*,
        };
    }
    _crate_internals {
        pub use super::term::_crate_internals::*;
    }
}
