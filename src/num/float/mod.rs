// devela::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

mod ext_float; // FloatExt
mod wrapper; // Float

// re-exports
crate::mod_path!(_c "../../../libs/base_core/src/num/float/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ext_float::*,
            wrapper::*,
        };

        // re-exports
        pub use super::_c::*;
        #[doc(inline)]
        pub use devela_base_core::{f32bits, f32bits_niche, f64bits, f64bits_niche, fsize};
        #[doc(inline)]
        pub use devela_base_num::{FloatConst};
    }
}
