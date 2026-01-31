// devela_base_core::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()] // public
#![doc = crate::_doc!(modules: crate::text; fmt)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: fmt)]
//

mod _reexport; // SYMLINK from /src/text/fmt/_reexport_core.rs

mod buf; // FmtWriter, format_buf!
mod cat; // fmtcat!
mod debug; // DebugWith
mod num; // FmtNum, FmtNumShape

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            buf::*,
            cat::*,
            debug::*,
            num::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
