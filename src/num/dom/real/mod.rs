// devela::num::dom::real
//
#![doc = crate::_DOC_NUM_DOM_REAL!()]
//

// mod fixed; //
mod float; // FloatExt, (Float, FloatConst, f[32|64]_bits, fsize)

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // fixed::_all::*,
            float::_all::*,
        };
    }
}
