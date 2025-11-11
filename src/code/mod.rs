// devela::code
//
//! Code reflective synthesis.
#![doc = crate::_doc!(modules: crate; code: error, marker, ops, panic, result, util)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends:
    any, clone, convert, default, error, hint, marker, ops, panic, result)]
//
// safety
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]

// re-exports
crate::mod_path!(_c "../../libs/base_core/src/code/reexports.rs");

mod any; // dynamic typing and reflection
pub(crate) mod default; // ConstDefault

pub mod error; // AllError, modular errors
pub mod intro; // Introspect
pub mod marker; // core::marker, TypeResource, TypeResourced, type_marker!, type_resource!
pub mod ops; // ::core::ops::*
pub mod panic; // Panic, set_panic_handler!
pub mod result; // AllError, Mismatch, OptRes, ValueQuant, serr, sok, …
pub mod util; // (utility macros and functions)

devela_base_core::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            _c::*,
            any::_all::*,
            default::ConstDefault,
        };

        // re-exports
        #[doc(inline)]
        pub use devela_base_core::code::{ConstDefaultCore, Lut, ScopeGuard};
    }
    _pub_mods {
        pub use super::{
            error::_all::*, intro::_all::*, marker::_all::*, ops::_all::*,
            panic::_all::*, result::_all::*, util::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::util::_crate_internals::*;
        pub(crate) use super::default::{Sealed as ConstDefaultSealed, impl_cdef};
    }
}
