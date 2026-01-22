// devela::work::process
//
#![doc = crate::_DOC_WORK_PROCESS!()]
//!
#![doc = crate::_doc!(extends: process)]
//

#[cfg(feature = "std")]
crate::items! {
    mod _reexport_std; // SYMLINK to /src/base/std/src/work/process/_reexport.rs

    mod cmd; // cmd!
    mod error; // ExitStatusError
    mod flow; // CommandFlow
    mod output; // OutputExt
    mod process; // ProcessExt
}

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "std")]
        pub use super::{
            cmd::*,
            error::*,
            flow::*,
            output::*,
            process::*,
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use {
            super::_reexport_std::*,
        };
    }
}
