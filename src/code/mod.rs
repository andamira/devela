// devela::code
//
//! Code reflective synthesis.
#![doc = crate::doc_!(modules: crate; code: error, marker, ops, panic, result, util)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends:
    any, clone, convert, default, error, hint, marker, ops, panic, result)]
//
// safety
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]

mod any; // dynamic typing and reflection
mod default; // ConstDefault, Default
mod guard; // ScopeGuard
mod reexports; // re-exported items

pub mod error; // AllError, modular errors
pub mod intro; // Introspect
pub mod marker; // core::marker, type_marker!, type_resource!, TypeResource, TypeResourced
pub mod ops; // ::core::ops::*
pub mod panic; // Panic, set_panic_handler!
pub mod result; // AllError, serr, sok, Mismatch, OptRes, ValueQuant…
pub mod util; // utility macros and functions

crate::items! { // structural access: _mods, _pub_mods, _internals, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _internals::*};
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods {
        pub use super::{any::_all::*, default::*, guard::*, reexports::*};
    }
    mod _pub_mods {
        pub use super::{
            error::_all::*, intro::_all::*, marker::_all::*, ops::_all::*,
            panic::_all::*, result::_all::*, util::_all::*,
        };
    }
    pub(super) mod _internals {
        pub(crate) use super::util::_internals::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_pub_mods::*, _mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{
            error::_always::*, marker::_always::*, panic::_always::*,
            reexports::*, result::_always::*, util::_always::*, };
    }
}
