// devela::sys::os
//
//! OS-specific.
#![doc = crate::doc_!(modules: crate::sys; os: linux)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: os)]
//

#[cfg(feature = "linux")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linux")))]
pub mod linux;

crate::items! { // structural access: _pub_mods, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods {
        #[cfg(feature = "linux")]
        pub use super::linux::_all::*;
    }
    pub(super) mod _all { #[allow(unused)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
