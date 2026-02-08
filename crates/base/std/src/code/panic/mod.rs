// devela_base_std::code::panic
//
#![doc = crate::_DOC_CODE_PANIC!()] // public
#![doc = crate::_doc!(modules: crate::code; panic)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//

mod _reexport; // SYMLINK from /src/code/panic/_reexport_std.rs

mod namespace; // Panic

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
