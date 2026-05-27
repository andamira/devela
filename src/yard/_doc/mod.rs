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
    // re-exported for documentation
    _mods {
        pub use super::{
            _crate_internals::*,
            // don't document abbrs or tags:
            _hidden::{_doc_meta, _doc_vendor, _tags},
        };
    }
    _crate_internals {
        pub use super::{
            availability::_doc_availability,
            doc::{_doc, _doc_miri_warn},
            location::_doc_location,
            size_of::_doc_size_of,
            fragments::*,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            // needed by _use_or_shim!
            vendor::_doc_vendor,
            // needed by rand_pcg!
            meta::_doc_meta,
            fragments::{_ABBR_PCG, _ABBR_PRNG},
            // needed by _tags!
            tags::*,
        };
    }
}
