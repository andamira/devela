// devela_base_core::code::marker
//
#![doc = crate::_DOC_CODE_MARKER!()]
//

mod _reexport; // SYMLINK from /src/code/marker/_reexport_core.rs

mod type_marker; // zero-cost generic type markers

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::type_marker::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
