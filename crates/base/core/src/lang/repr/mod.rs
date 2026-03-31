// devela_base_core::lang::repr
//
#![doc = crate::_DOC_LANG_REPR!()] // public
#![doc = crate::_doc!(modules: crate::lang; repr)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//!
//! Declarative languages for describing data, documents, and structure.
//

mod data; // csv, json, ini, toml, usv, yaml, …
// mod item; // ReprITem
// mod markup; // html, svg, xml, …
// mod style; // css, sass, …
// mod text; // md, mdx, latex, …

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            data::_all::*,
            // item::*,
            // markup::_all::*,
            // style::_all::*,
            // text::_all::*,
        };
    }
}
