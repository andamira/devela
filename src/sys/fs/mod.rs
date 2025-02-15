// devela::sys::fs
//
//! Filesystem abstractions.
//!
#![doc = crate::doc_!(extends: fs, path)]
#![doc = crate::doc_!(newline)]
//

mod path;
mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{path::_all::*, reexports::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
