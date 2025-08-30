// devela::code::marker
//
#![doc = crate::_DOC_CODE_MARKER!()]
//!
#![doc = crate::_doc!(extends: marker)]
//

crate::mod_path!(_c "../../../libs/base/src/code/marker/reexports.rs");

mod type_resource; // zero-cost type-safe resource markers

crate::structural_mods! { // _mods
    _mods {
        pub use super::{_c::*, type_resource::*};

        #[doc(inline)]
        pub use devela_base::code::marker::type_marker;
    }
}
