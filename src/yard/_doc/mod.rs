// devela::yard::_doc
//
//! Defines private doc meta helpers.
//

mod availability; // _doc_availability!
mod doc; // _doc!, _doc_warn_miri!
mod location; // _doc_location!
mod meta; // _doc_meta!
mod size_of; // _doc_test_size_of!
mod vendor; // _doc_vendor!
mod warn; // _doc_warn_miri!

mod fragments; // _DOC_*!
mod tags; // _ABBR_!*, _TAG_*!, _tags!

crate::structural_mods! { // _mods, _crate_internals, _hidden
    // re-exported for documentation
    _mods {
        pub use super::{
            _crate_internals::*,
            // don't document: _ABBR_*, _TAG_*,
            _hidden::{_doc_meta, _doc_vendor, _tags},
        };
    }
    _crate_internals {
        pub use super::{
            availability::_doc_availability,
            doc::_doc,
            location::_doc_location,
            size_of::_doc_test_size_of,
            warn::_doc_warn_miri,
            fragments::*,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            // needed by _use_or_shim!:
            vendor::_doc_vendor,
            // needed by rand_pcg!:
            meta::_doc_meta,
            fragments::{_ABBR_PCG, _ABBR_PRNG},
            // needed by _tags!:
            tags::*,
        };
    }
}
