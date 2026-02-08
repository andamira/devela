// devela_base_core::code::any
//
#![doc = crate::_DOC_CODE_ANY!()] // private
#![doc = crate::_doc!(modules: crate::code; any)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: any)]
//

mod _reexport; // SYMLINK from /src/code/any/_reexport_core.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
