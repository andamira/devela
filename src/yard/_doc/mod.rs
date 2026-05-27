// devela::yard::_doc
//
//! Defines private doc meta helpers.
//

mod availability; // _doc_availability!
mod doc; // _doc!, _doc_miri_warn!
mod location; // _doc_location!
mod meta; // _doc_meta!
mod size_of; // _doc_size_of!
mod vendor; // _doc_vendor!

mod fragments; // _DOC_*!
mod tags; // _ABBR_!*, _TAG_*!, _tags!

crate::structural_mods! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            _crate_internals::*,
            _hidden::*,
        };
    }
    _crate_internals {
        pub use super::{
            availability::_doc_availability,
            doc::{_doc, _doc_miri_warn},
            location::_doc_location,
            meta::_doc_meta,
            size_of::_doc_size_of,
            fragments::*,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            vendor::_doc_vendor,
            tags::*,
        };
    }
}
