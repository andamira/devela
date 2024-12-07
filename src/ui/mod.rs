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

// structural access
crate::items! {
    mod doc_inline {
        #[cfg(_ui_·)]
        pub use super::error::*;
        #[cfg(feature = "layout")]
        pub use super::layout::all::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
