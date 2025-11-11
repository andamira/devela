// devela_base_core::data::list::array
//
#![doc = crate::_DOC_DATA_LIST_ARRAY!()]
//!
#![doc = crate::_doc!(extends: array, vec)]
//

mod ext; // ExtArray, ArrayFmt
mod from; // ArrayFrom
mod init; // array_init!
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ext::*,
            from::*,
            init::*,
            reexports::*,
        };
    }
}
