// devela::lang::js
//
//! JavaScript interfacing.
//

mod types; // Js, JsEvent, JsPermission*, JsWorker...

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
        pub use super::types::*;

        #[cfg(feature = "unsafe_ffi")]
        pub use super::reexport::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
