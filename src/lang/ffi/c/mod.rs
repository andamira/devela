// devela::lang::ffi::c
//
#![doc = crate::_DOC_LANG_FFI_C!()]
//

crate::mod_path!(_c "../../../../libs/base/src/lang/ffi/c/reexports.rs");
crate::mod_path!(alloc _a "../../../../libs/base_alloc/src/lang/ffi/c/reexports.rs");

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::_c::*;
        #[cfg(feature = "alloc")]
        pub use super::_a::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
