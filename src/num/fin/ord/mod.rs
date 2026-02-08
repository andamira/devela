// devela::num::fin::ord
//
#![doc = crate::_DOC_NUM_FIN_ORD!()] // public
#![doc = crate::_doc!(modules: crate::num::fin; ord)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: cmp)]
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/num/fin/ord/_reexport.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport_core::*;
        #[doc(inline)]
        pub use devela_base_core::num::fin::{ // ord
            Cmp, cmp, Order,
        };
    }
}
