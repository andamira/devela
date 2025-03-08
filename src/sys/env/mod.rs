// devela::sys::env
//
//! Inspection and manipulation of the process’s environment.
//!
#![doc = crate::doc_!(extends: env)]
//

mod arg;
mod namespace;
mod reexports;

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod app;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::{arg::*, namespace::*, reexports::*};

        #[cfg(feature = "std")]
        pub use super::app::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
