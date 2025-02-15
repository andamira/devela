// devela::sys::fs
//
//! Filesystem abstractions.
//!
#![doc = crate::doc_!(extends: fs, path)]
#![doc = crate::doc_!(newline)]
//

mod path;
mod reexports;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
mod fs_path;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{path::_all::*, reexports::*};

        #[cfg(feature = "std")]
        pub use super::fs_path::*;
        // WIPZONE
        // pub use super::ext::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIP ZONE
// mod ext; // ExtFile
