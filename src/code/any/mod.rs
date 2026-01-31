// devela::code::any
//
#![doc = crate::_DOC_CODE_ANY!()] // private
#![doc = crate::_doc!(modules: crate::code; any)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(extends: any)]
//

mod _reexport_core; // SYMLINK TO /src/base/core/src/code/any/_reexport.rs

mod ext;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::ext::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
