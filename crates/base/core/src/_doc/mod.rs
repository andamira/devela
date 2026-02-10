// devela_base_core::_doc
//
//! Internal doc helpers.
//

mod _links; // _DOCLINK_*!
mod _mod_docs; // _DOC_*!
mod _tags; // EMOJI_*!, _TAG_*! _tags!

pub(crate) mod _doc; // _doc!, _doc_availability!, _doc_miri_warn!

crate::structural_mods! { // _workspace_internals
    _workspace_internals {
        pub use super::{
            _doc::{_doc, _doc_availability, _doc_location, _doc_miri_warn},
            _links::*,
            _mod_docs::*,
            _tags::*,
        };
    }
}
