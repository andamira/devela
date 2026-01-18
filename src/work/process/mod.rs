// devela::work::process
//
#![doc = crate::_DOC_WORK_PROCESS!()]
//!
#![doc = crate::_doc!(extends: process)]
//

#[cfg(feature = "std")]
crate::items! {
    mod _reexport_std; // SYMLINK to /libs/base_std/src/work/process/_reexport.rs

    mod cmd; // cmd!
    mod ext; // ProcessExt
    mod pipe; // Pipeline
}

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "std")]
        pub use super::{
            cmd::*,
            ext::*,
            pipe::*,
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use {
            super::_reexport_std::*,
        };
    }
}
