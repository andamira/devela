// devela::yard::_doc
//
//! Defines private doc meta helpers.
//

mod _doc; // _doc!, _doc_meta!, _doc_location!…
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
            _doc::{
                _doc, _doc_meta, _doc_availability, _doc_location, _doc_miri_warn, _doc_size_of,
            },
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            _doc::{_doc_vendor},
            fragments::*,
            tags::*,
        };
    }
}
