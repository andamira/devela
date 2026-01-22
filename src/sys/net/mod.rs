// devela::sys::net
//
#![doc = crate::_DOC_SYS_NET!()]
//!
#![doc = crate::_doc!(extends: net)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/sys/net/_reexport.rs
#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /src/base/std/src/sys/net/_reexport.rs

// #[cfg(feature = "std")]
// mod http_server;

crate::structural_mods! { // _mods, _reexports
    _mods {
        // #[cfg(feature = "std")]
        // pub use super::http_server::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
