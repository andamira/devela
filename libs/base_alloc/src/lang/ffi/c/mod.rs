// devela_base_alloc::lang::ffi::c
//
#![doc = crate::_DOC_LANG_FFI_C!()]
//

mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{
            reexports::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
