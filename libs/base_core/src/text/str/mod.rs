// devela_base_core::text::str
//
#![doc = crate::_DOC_TEXT_STR!()]
//

mod nonul; // StringNonul
// mod _wip_sixbit; WIP
mod str; // Str
mod u; // StringU8, StringU16, StringU32, StringUsize

mod _reexport; // SYMLINK from /src/text/str/_reexport_core.rs

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            nonul::*,
            // _wip_sixbit::*;
            str::Str,
            u::*,
            _reexport::*,
        };
    }
}
