// devela::code::any
//
#![doc = crate::_DOC_CODE_ANY!()]
// #![doc = crate::_doc!(extends: any)]
// #![doc = crate::_doc!(modules: crate::code; any)]
// #![doc = crate::_doc!(newline)]
//

mod _reexport_core; // SYMLINK TO /libs/base_core/src/code/any/_reexport.rs

mod ext;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::ext::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
