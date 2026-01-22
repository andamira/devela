// devela::sys::env
//
#![doc = crate::_DOC_SYS_ENV!()]
//!
#![doc = crate::_doc!(extends: env)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/sys/env/_reexport.rs
#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /src/base/std/src/sys/env/_reexport.rs

mod arg;
mod namespace;

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod app;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            arg::_all::*,
            namespace::*,
        };

        #[cfg(feature = "std")]
        pub use super::app::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
