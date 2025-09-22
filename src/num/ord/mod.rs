// devela::num::ord
//
#![doc = crate::_DOC_NUM_ORD!()]
// #![doc = crate::_doc!(extends: cmp)]
// #![doc = crate::_doc!(modules: crate::num; cmp)]
// #![doc = crate::_doc!(newline)]

// re-exports
crate::mod_path!(_c "../../../libs/base_core/src/num/ord/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        // re-exports
        pub use super::_c::*;
        #[doc(inline)]
        pub use devela_base_core::Cmp;
    }
}
