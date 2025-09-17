// devela_base_core::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()]
// #![doc = crate::doc_!(extends: fmt)]
//

mod buf; // FmtWriter, format_buf!
mod num_to_str; // NumToStr
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            buf::*,
            num_to_str::*,
            reexports::*,
        };
    }
}
