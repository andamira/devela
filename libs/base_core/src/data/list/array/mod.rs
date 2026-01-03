// devela_base_core::data::list::array
//
#![doc = crate::_DOC_DATA_LIST_ARRAY!()]
//!
#![doc = crate::_doc!(extends: array, vec)]
//

mod _reexport; // SYMLINK from /src/data/list/array/_reexport_core.rs

// mod define; // define_array, ArrayIter, ArrayIterMut WIP
// mod define_usize; // Array WIP TEMP
mod ext; // ArrayExt, ArrayFmt
mod from; // ArrayFrom
mod init; // array_init!

crate::structural_mods! { // _mods, _reexports
    _mods {
        // #[cfg(all(not(base_safe_data), feature = "unsafe_array"))]
        // pub use super::define::ArrayIterMut; // WIP

        pub use super::{
            // define::{define_array, ArrayIter},  // WIP
            // define_usize::{Array as ArrayWip}, // WIP TEMP
            ext::*,
            from::*,
            init::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
