// devela_base_alloc::text::str
//
#![doc = crate::_DOC_TEXT_STR!()] // public
#![doc = crate::_doc!(modules: crate::text; str)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: str, string)]
//

mod _reexport; // SYMLINK from /src/text/str/_reexport_alloc.rs

crate::structural_mods! { // _mods
    _mods {
        pub use super::_reexport::*;
    }
}
