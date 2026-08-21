// devela/src/code/mod.rs
//
#![doc = crate::_DOC_CODE!()] // public, root
#![doc = crate::_DOC_CODE_MODULES!()]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends:
    any, clone, convert, default, hint, marker, ops, panic, range, result)]
//!
//! These facilities describe concerns shared across otherwise unrelated domains:
//! type identity and conversion, initialization and markers, operational and
//! outcome semantics, source provenance, and code-oriented tooling.
//!
//! [`any`], [`convert`], [`init`], [`marker`], [`ops`], and [`result`] provide
//! reusable language-level semantics. [`hint`] and [`panic`][mod@panic] concern
//! execution behavior, [`source`] describes provenance and inclusion, and [`util`]
//! gathers code-authoring utilities that do not have a more specific semantic home.
//
// safety
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_CODE_MODULES =
    crate::_doc!(modules: crate; code:
        any, convert, hint, init, marker, ops, panic, result, source, util);
}

pub mod any; // Dynamic typing, type identity, and type inspection
pub mod convert; // Type conversion and adaptation
pub mod hint; // Compiler and execution hints
pub mod init; // Default and const-friendly initialization
pub mod marker; // Marker types, traits, and macros
pub mod ops; // Operational syntax, semantics, and composition
pub mod panic; // Panic hooks, unwinding, and abort strategies
pub mod result; // Generic outcome and resolution types
pub mod source; // Source-code location, provenance, and inclusion
pub mod util; // Cross-cutting code and macro utilities

util::structural_mods! { // _pub_mods, _reexports, _crate_internals
    _pub_mods {
        pub use super::{
            any::_all::*,
            convert::_all::*,
            hint::_all::*,
            init::_all::*,
            marker::_all::*,
            ops::_all::*,
            panic::_all::*,
            result::_all::*,
            source::_all::*,
            util::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            init::ConstInit,
            result::unwrap,
            source::CodeLocation,
            util::const_assert,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_CODE_MODULES;
        pub(crate) use super::util::_crate_internals::*;
        pub(crate) use super::init::_crate_internals::*;
    }
    _hidden {
        pub use super::util::_hidden::*;
    }
}
