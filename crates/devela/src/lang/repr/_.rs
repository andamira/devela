//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_LANG_REPR!()] // public
#![doc = crate::_doc!(modules: crate::lang; repr)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//!
//! Declarative languages for describing data, documents, and structure.
//

crate::mods_in! {
    // mod_ data; // json, toml, ini, csv, usv, …
    // mod item; // ReprITem
    // mod_ markup; // html, css, xml
    // mod_ style; // css, sass, …
    // mod_ text; // MAYBE: hybrid // yaml, latex, …
}
crate::mods_out! { // _pub_mods, _crate_internals
    _mods {
        // pub use super::{
        //     data::_all::*,
        //     item::ReprItem,
        //     markup::_all::*,
        //     style::_all::*,
        //     text::_all::*,
        // };
    }
}
