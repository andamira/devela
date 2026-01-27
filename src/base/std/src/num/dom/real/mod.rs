// devela_base_std::num::dom::real
//
#![doc = crate::_DOC_NUM_DOM_REAL!()]
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
