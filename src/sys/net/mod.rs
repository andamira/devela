// devela::sys::net
//
#![doc = crate::_DOC_SYS_NET!()]
//!
#![doc = crate::_doc!(extends: net)]
//

mod reexports;

// WIPZONE
// #[cfg(feature = "std")]
// mod http_server;

crate::structural_mods! { // _mods
    _mods {
        pub use super::reexports::*;
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::http_server::*;
    }
}
