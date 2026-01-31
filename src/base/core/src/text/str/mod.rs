// devela_base_core::text::str
//
#![doc = crate::_DOC_TEXT_STR!()] // public
#![doc = crate::_doc!(modules: crate::text; str)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: str, string)]
//

mod namespace; // Str
mod nonul; // StringNonul
// mod _wip_sixbit; WIP
mod u; // StringU8, StringU16, StringU32, StringUsize

mod _reexport; // SYMLINK from /src/text/str/_reexport_core.rs

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::Str,
            nonul::*,
            // _wip_sixbit::*;
            u::*,
            _reexport::*,
        };
    }
}
