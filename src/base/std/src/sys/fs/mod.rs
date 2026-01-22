// devela_base_std::sys::fs
//
#![doc = crate::_DOC_SYS_FS!()]
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
