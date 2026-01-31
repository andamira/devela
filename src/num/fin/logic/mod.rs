// devela::num::fin::logic
//
#![doc = crate::_DOC_NUM_FIN_LOGIC!()] // public
#![doc = crate::_doc!(modules: crate::num::fin; logic)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::structural_mods! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::num::fin::logic::{
            ConstBool, False, True, const_bool,
        };
    }
}
