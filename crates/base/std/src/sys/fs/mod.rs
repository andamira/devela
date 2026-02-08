// devela_base_std::sys::fs
//
#![doc = crate::_DOC_SYS_FS!()] // public
#![doc = crate::_doc!(modules: crate::sys; fs)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: fs, path)]
//

mod _reexport;

mod path;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::path::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
