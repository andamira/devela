// devela::num::ord
//
#![doc = crate::_DOC_NUM_ORD!()]
// #![doc = crate::_doc!(extends: cmp)]
// #![doc = crate::_doc!(modules: crate::num; cmp)]
// #![doc = crate::_doc!(newline)]
//

// reexports
crate::mod_path!(_c "../../../libs/base/src/num/ord/reexports.rs");

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        #[doc(inline)]
        pub use devela_base::Compare;

        pub use super::_c::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
