// devela/src/num/dom/real/_.rs
//
#![doc = crate::_DOC_NUM_DOM_REAL!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; real)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod fixed; //
    mod_ float; // FloatExt, (Float, FloatConst, f[32|64]_bits, fsize)
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            // fixed::_all::*,
            float::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::float::_crate_internals::*;
    }
}
