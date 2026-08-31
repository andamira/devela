// devela/src/error/_.rs
//
#![doc = crate::_DOC_ERROR!()] // public, root
#![doc = crate::_DOC_ERROR_MODULES!()]
#![doc = crate::_doc!(flat:"error")]
#![doc = crate::_doc!(extends: backtrace, error)]
//
// safety
// #![cfg_attr(feature = "safe_error", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_ERROR_MODULES =
    crate::_doc!(modules: crate; error: data, num, text); // media, ui
}

crate::mods_in! {
    mod _reexport_core;
    #[cfg(feature = "std")]
    mod _reexport_std;

    // mod context; // ContextualError WIP
    mod kind; // reusable failure categories
    mod macros; // define_error!

    pub mod_ data; // Data-related error types
    // pub mod_ media; // Media-related error types.
    pub mod_ num; // Numeric-related error types.
    pub mod text; // Text-related error types.
    // pub mod_ ui; // WIP
}
crate::mods_out! { // _mods, _pub_mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            // context::*,
            kind::*,
            macros::define_error,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            data::_all::*,
            // media::_all::*,
            num::_all::*,
            text::*,
            // media::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_ERROR_MODULES,
        };
    }
}
