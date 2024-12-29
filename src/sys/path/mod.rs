// devela::sys::path
//
//! Paths.
#![doc = crate::doc_!(extends: path)]
#![doc = crate::doc_!(modules: crate::sys; path)]
#![doc = crate::doc_!(newline)]
//!
//

mod reexports;

#[cfg(feature = "std")]
mod project;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::reexports::*;

        #[cfg(feature = "std")]
        pub use super::project::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
