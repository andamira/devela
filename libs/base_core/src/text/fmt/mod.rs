// devela_base_core::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()]
// #![doc = crate::doc_!(extends: fmt)]
//

mod buf; // FmtWriter, format_buf!
mod cat; // fmtcat!
mod debug; // DebugWith
mod num; // FmtNum, FmtNumShape
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            buf::*,
            cat::*,
            debug::*,
            num::_all::*,
            reexports::*,
        };
    }
}
