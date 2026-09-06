// devela/src/yard/_doc/_.rs
//
//! Defines private doc meta helpers.
//

crate::mods_in! {
    mod availability; // _doc_availability!
    mod doc; // _doc!, _doc_warn_miri!
    mod location; // _doc_location!
    mod meta; // _doc_meta!
    mod test_size_of; // _doc_test_size_of!
    mod vendor; // _doc_vendor!
    mod warn; // _doc_warn_miri!
}
crate::mods_out! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            // RETHINK
            _crate_internals::*,
            _hidden::*,
        };
    }
    _crate_internals {
        pub use super::{
            availability::_doc_availability,
            doc::_doc,
            warn::_doc_warn_miri,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            location::_doc_location,
            // needed by _use_or_shim!:
            vendor::_doc_vendor,
            // needed by rand_pcg!:
            meta::_doc_meta,
            test_size_of::_doc_test_size_of,
        };
    }
}
