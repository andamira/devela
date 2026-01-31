// devela_base_std::num::dom::real
//
#![doc = crate::_DOC_NUM_DOM_REAL!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; real)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod float; // FloatStd

crate::structural_mods! { // _mods, _workspace_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            float::_all::*,
        };
    }
}
