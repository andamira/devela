// devela_base_core::code
//
#![doc = crate::_DOC_CODE!()]
#![doc = crate::_doc!(modules: crate; code: error, marker, ops, panic, result, util)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends:
    any, clone, convert, default, error, hint, marker, ops, panic, result)]
//
// safety
#![cfg_attr(base_safe_code, forbid(unsafe_code))]

mod any; // dynamic typing and reflection
mod const_init; // ConstInitCore, <_impl_init!>
mod guard; // ScopeGuard
mod lut; // Lut RETHINK
mod site; // CodeLocation, CodeSpan

mod reexports;

pub mod error; // general errors definitions
pub mod marker; // core::marker, type_marker!, type_resource!, TypeResource, TypeResourced
pub mod ops; // ::core::ops::*
pub mod panic;
pub mod result; // utility macros and functions
pub mod util; // utility macros and functions

util::structural_mods! { // _mods, _pub_mods, _workspace_internals
    _mods {
        pub use super::{
            any::_all::*,
            const_init::ConstInitCore,
            guard::*,
            lut::*,
            reexports::*,
            site::*,
        };
    }
    _pub_mods {
        pub use super::{
            error::_all::*, marker::_all::*, ops::_all::*,
            panic::_all::*, result::_all::*, util::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::util::_crate_internals::*;
    }
    _workspace_internals {
        pub/*workspace*/ use super::const_init::_impl_init;
        pub/*workspace*/ use super::util::_workspace_internals::*;
    }
}
