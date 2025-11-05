// devela_base_core::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()]
// #![doc = crate::doc_!(extends: fmt)]
//

mod buf; // FmtWriter, format_buf!
mod cat; // fmtcat!
mod num; // FmtNum
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            buf::*,
            cat::*,
            num::*,
            reexports::*,
        };
    }
}
