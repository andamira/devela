// devela::sys::net
//
#![doc = crate::_DOC_SYS_NET!()]
//!
#![doc = crate::doc_!(extends: net)]
//

mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::reexports::*;
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::http_server::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)] pub use super::_mods::*;
    }
}
// WIPZONE
// #[cfg(feature = "std")]
// mod http_server;
