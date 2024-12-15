// devela::sys::path
//
//! Paths.
#![doc = crate::doc_!(extends: path)]
#![doc = crate::doc_!(modules: crate::sys; path)]
#![doc = crate::doc_!(newline)]
//!
//

#[cfg(all(feature = "sys", feature = "std"))]
mod project;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        #[cfg(all(feature = "sys", feature = "std"))]
        pub use super::project::*;
    }
    pub(super) mod _all {
        #[allow(unused_imports, reason = "feature-gated")]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
