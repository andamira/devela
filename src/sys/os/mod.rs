// devela::sys::os
//
//! OS-specific.
#![doc = crate::doc_!(modules: crate::sys; os: linux)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: os)]
//

#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;

crate::items! { // structural access: _pub_mods, _all
    #[allow(unused)]
    pub use _pub_mods::*;

    mod _pub_mods {
        #[doc(hidden)] #[doc(no_inline)]
        #[cfg(feature = "linux")]
        pub use super::linux::_all::*;
    }
    pub(super) mod _all { #[allow(unused)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
