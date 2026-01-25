// devela::code
//
#![doc = crate::_DOC_CODE!()]
#![doc = crate::_DOC_CODE_MODULES!()]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(br+lf)]
#![doc = crate::_doc!(extends:
    any, clone, convert, default, error, hint, marker, ops, panic, result)]
//
// safety
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_CODE_MODULES =
    crate::_doc!(modules: crate; code: error, marker, ops, panic, result, util);
}

mod _reexport_core; // SYMLINK to /src/base/core/src/code/_reexport.rs

mod any; // dynamic typing and reflection
mod const_init; // ConstInit
mod intro; // Introspect

pub mod error; // AllError, modular errors
pub mod marker; // core::marker, TypeResource, TypeResourced, type_marker!, type_resource!
pub mod ops; // ::core::ops::*
pub mod panic; // Panic, set_panic_handler!
pub mod result; // AllError, Mismatch, OptRes, ValueQuant, serr, sok, …
pub mod util; // (utility macros and functions)

devela_base_core::structural_mods! { // _mods, _pub_mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            any::_all::*,
            const_init::ConstInit,
            intro::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            error::_all::*,
            marker::_all::*,
            ops::_all::*,
            panic::_all::*,
            result::_all::*,
            util::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[doc(inline)]
        pub use devela_base_core::code::{
            CodeLocation, CodeSpan,
            ConstInitCore,
            ScopeGuard,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_CODE_MODULES;
        pub(crate) use super::util::_crate_internals::*;
        pub(crate) use super::const_init::{Sealed as ConstInitSealed};
    }
}
