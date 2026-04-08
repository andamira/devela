// devela::lang::repr
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_LANG_REPR!()] // public
#![doc = crate::_doc!(modules: crate::lang; repr)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//!
//! Declarative languages for describing data, documents, and structure.
//

// mod data; // json, toml, ini, csv, usv, …
// mod item; // ReprITem
// mod markup; // html, css, svg, xml
// mod style; // css, sass, …
// mod text; // MAYBE: hybrid // yaml, latex, …

crate::structural_mods! { // _pub_mods, _crate_internals
    _mods {
        // pub use super::{
        //     item::*,
        //     data::_all::*,
        //     markup::_all::*,
        //     style::_all::*,
        //     text::_all::*,
        // };
    }
}
