// devela::num::dom::real
//
#![doc = crate::_DOC_NUM_DOM_REAL!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; real)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

// mod fixed; //
mod float; // FloatExt, (Float, FloatConst, f[32|64]_bits, fsize)

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            // fixed::_all::*,
            float::_all::*,
        };
    }
    _crate_internals {
        pub use super::float::_crate_internals::*;
    }
}
