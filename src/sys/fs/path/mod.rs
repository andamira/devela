// devela::sys::fs::path
//
//! Cross-platform path manipulation.
//!
#![doc = crate::doc_!(extends: path)]
#![doc = crate::doc_!(newline)]
//

mod reexports;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
mod ext; // ExtPath

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::reexports::*;

        #[cfg(feature = "std")]
        pub use super::ext::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
