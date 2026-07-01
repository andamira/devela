// devela/src/error/mod.rs
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

mod _reexport_core;
#[cfg(feature = "std")]
mod _reexport_std;

// mod context; // ContextualError WIP
mod kind; // reusable failure categories
mod macros; // define_error!

pub mod data; // Data-related error types
// pub mod media; // Media-related error types.
pub mod num; // Numeric-related error types.
pub mod text; // Text-related error types.
// pub mod ui; // WIP

crate::structural_mods! { // _mods, _pub_mods, _reexports, _crate_internals
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
