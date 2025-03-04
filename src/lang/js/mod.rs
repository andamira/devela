// devela::lang::js
//
//! Javascript interfacing.
//

mod definitions; // Js, JsEvent, JsPermission*

#[cfg(feature = "unsafe_ffi")]
crate::items! {
    mod reexport; // js_reexport!

    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ffi")))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(target_arch = "wasm32")))]
    #[cfg(not(windows))]
    mod web_api;
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::definitions::*;

        #[cfg(feature = "unsafe_ffi")]
        pub use super::reexport::*;

        // WIPZONE
        // #[cfg(feature = "alloc")]
        // pub use super::bson::*;
        // #[cfg(feature = "std")]
        // pub use super::json::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// #[cfg(feature = "alloc")]
// mod bson;
// #[cfg(feature = "std")]
// mod json;
