// devela::code::marker
//
#![doc = crate::_DOC_CODE_MARKER!()]
//!
#![doc = crate::_doc!(extends: marker)]
//

crate::mod_path!(_c "../../../libs/base/src/code/marker/reexports.rs");

mod type_resource; // zero-cost type-safe resource markers

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{_c::*, type_resource::*};

        pub use devela_base::code::marker::type_marker;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
