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

crate::mod_path!(_c "../../libs/base_core/src/code/reexports.rs");

mod any; // dynamic typing and reflection
mod default; // ConstDefault, Default

pub mod error; // AllError, modular errors
pub mod intro; // Introspect
pub mod marker; // core::marker, type_marker!, type_resource!, TypeResource, TypeResourced
pub mod ops; // ::core::ops::*
pub mod panic; // Panic, set_panic_handler!
pub mod result; // AllError, serr, sok, Mismatch, OptRes, ValueQuant…
pub mod util; // utility macros and functions

devela_base_core::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{_c::*, any::_all::*, default::*};

        #[doc(inline)]
        pub use devela_base_core::code::{Lut, ScopeGuard};
    }
    _pub_mods {
        pub use super::{
            error::_all::*, intro::_all::*, marker::_all::*, ops::_all::*,
            panic::_all::*, result::_all::*, util::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::util::_crate_internals::*;
    }
}
