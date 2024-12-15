// devela::text::fmt
//
//! Formatting strings.
//!
#![doc = crate::doc_!(extends: fmt)]
//

mod reexports;
mod buf;

#[cfg(feature = "fmt")]
mod num_to_str;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{buf::*, reexports::*};

        #[cfg(feature = "fmt")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "fmt")))]
        pub use super::num_to_str::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
