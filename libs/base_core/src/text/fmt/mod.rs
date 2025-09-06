// devela_base_core::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()]
// #![doc = crate::doc_!(extends: fmt)]
//

mod num_to_str;
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            num_to_str::*,
            reexports::*,
        };
    }
}
