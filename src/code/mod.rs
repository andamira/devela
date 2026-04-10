// devela::code
//
#![doc = crate::_DOC_CODE!()] // public, root
#![doc = crate::_DOC_CODE_MODULES!()]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends:
    any, clone, convert, default, error, hint, marker, ops, panic, result)]
//
// safety
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_CODE_MODULES =
    crate::_doc!(modules: crate; code: error, marker, ops, panic, result, util);
}

mod _reexport_core;

mod any; // dynamic typing and reflection
mod const_init; // ConstInit
mod guard; // ScopeGuard
mod intro; // Introspect
mod site; // CodeLocation, CodeSpan

pub mod error; // AllError, modular errors
pub mod marker; // core::marker, TypeResource, TypeResourced, type_marker!, type_resource!
pub mod ops; // ::core::ops::*
pub mod panic; // Panic, set_panic_handler!
pub mod result; // AllError, Mismatch, OptRes, ValueQuant, serr, sok, …
pub mod util; // (utility macros and functions)

util::structural_mods! { // _mods, _pub_mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            any::_all::*,
            const_init::ConstInit, // TEMP:merge
            guard::*,
            intro::_all::*,
            site::*,
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
    }
    _crate_internals {
        pub(crate) use super::_DOC_CODE_MODULES;
        pub(crate) use super::util::_crate_internals::*;
        pub(crate) use super::const_init::_impl_init;
    }
    _hidden {
        pub use super::util::_hidden::*;
    }
}
