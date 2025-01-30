// devela::ui
//
//! User interface functionality.
#![doc = crate::doc_!(modules: crate; ui: layout, service)]
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]

#[cfg(ui··)]
crate::items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(ui··)))]
    mod error;

    #[cfg_attr(feature = "nightly_doc", doc(cfg(ui··)))]
    pub mod service; // UiService*, ...
}

#[cfg(feature = "layout")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "layout")))]
pub mod layout;

crate::items! { // structural access: _mods, _all,
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods {
        #[cfg(ui··)]
        pub use super::error::*;
    }
    pub(super) mod _pub_mods { #![allow(unused)]
        #[cfg(feature = "layout")]
        pub use super::layout::_all::*;

        #[cfg(ui··)]
        pub use super::service::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
