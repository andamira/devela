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

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        #[cfg(_ui_·)]
        pub use super::error::*;
        #[cfg(feature = "layout")]
        pub use super::layout::all::*;
    }
    pub(super) mod all { #![allow(unused)]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
    }
}
