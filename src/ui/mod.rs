// devela::ui
//
//! User interface functionality.
#![doc = crate::doc_!(modules: crate; ui: layout)]
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]

#[cfg(_ui_·)]
crate::items! {
    use crate::items;

    #[cfg_attr(feature = "nightly_doc", doc(cfg(_ui_·)))]
    mod error;
    #[doc(inline)]
    pub use error::*;
}

#[cfg(feature = "layout")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "layout")))]
    pub mod layout;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use layout::all::*;
}

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    #[cfg(_ui_·)]
    pub use super::error::*;

    #[doc(inline)]
    #[cfg(feature = "layout")]
    pub use super::layout::all::*;
}
