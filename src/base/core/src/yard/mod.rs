// devela_base_core::yard
//
//! Internal scaffolding and misc. machinery.
//!
//! > This space exists so the rest can be clean.
//

mod _env; // __dbg!, __std!, _std_core!
mod _reexport_macro; // _reexport!
mod _use; // _use!

pub mod _dep;

crate::structural_mods! { // _crate_internals, _workspace_internals
    _crate_internals {
        pub(crate) use super::_env::*;
    }
    _workspace_internals {
        pub use super::{
            _use::_use,
            _reexport_macro::_reexport,
        };
    }
}
