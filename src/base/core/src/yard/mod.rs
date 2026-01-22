// devela_base_core::yard
//
//! Internal scaffolding and machinery.
//!
//! > This space exists so the rest can be clean.
//

mod _doc; // _doc!, _doc_availability!, _doc_miri_warn!
mod _env; // __dbg!, __std!, _std_core!
mod _reexport_macro; // _reexport!
mod _use; // _use!

// IMPROVE
mod _links; // _DOCLINK_*!
mod _mod_docs; // _DOC_*!
mod _tags; // EMOJI_*!, _TAG_*! _tags!

crate::structural_mods! { // _crate_internals, _workspace_internals
    _crate_internals {
        pub(crate) use super::_env::*;
    }
    _workspace_internals {
        pub use super::{
            _doc::{_doc, _doc_availability, _doc_location, _doc_miri_warn},
            _use::_use,
            _links::*,
            _mod_docs::*,
            _reexport_macro::_reexport,
            _tags::*,
        };
    }
}
