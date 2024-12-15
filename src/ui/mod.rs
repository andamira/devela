// devela::ui
//
//! User interface functionality.
#![doc = crate::doc_!(modules: crate; ui: layout)]
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]

#[cfg(_ui_·)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_ui_·)))]
mod error;

#[cfg(feature = "layout")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "layout")))]
pub mod layout;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        #[cfg(_ui_·)]
        pub use super::error::*;
        #[cfg(feature = "layout")]
        pub use super::layout::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
