// devela::work::process
//
#![doc = crate::_DOC_WORK_PROCESS!()]
//!
#![doc = crate::_doc!(extends: process)]
//

crate::mod_path!(std _s "../../../libs/base_std/src/work/process/reexports.rs");

#[cfg(feature = "std")]
mod ext; // ExtProcess

crate::structural_mods! { // _mods, _always
    _mods {
        #[cfg(feature = "std")]
        pub use super::{_s::*, ext::*};
    }
    _always {
        #[cfg(feature = "std")]
        pub use super::{_s::*, ext::*};
    }
}
