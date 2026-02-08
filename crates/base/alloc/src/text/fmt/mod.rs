// devela_base_alloc::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()] // public
#![doc = crate::_doc!(modules: crate::text; fmt)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: fmt)]
//

mod _reexport; // SYMLINK from /src/text/fmt/_reexport_alloc.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
