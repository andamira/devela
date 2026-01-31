// devela::code::marker
//
#![doc = crate::_DOC_CODE_MARKER!()] // public
#![doc = crate::_doc!(modules: crate::code; marker)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: marker)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/code/marker/_reexport.rs

mod type_resource; // zero-cost type-safe resource markers

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::type_resource::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[doc(inline)]
        pub use devela_base_core::code::marker::type_marker;
    }
}
