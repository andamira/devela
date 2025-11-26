// devela::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

mod ext_float; // FloatExt
mod float_const; // FloatConst
mod wrapper; // Float

crate::mod_path!(_c "../../../libs/base_core/src/num/float/reexports.rs");

crate::structural_mods! { // _mods, crate_internals
    _mods {
        pub use super::{
            ext_float::*,
            float_const::*,
            wrapper::*,
        };

        /* re-exports */
        pub use super::_c::*;
        // aliases
        #[doc(inline)]
        pub use devela_base_core::{f32bits, f32bits_niche, f64bits, f64bits_niche, fsize};
    }
}
