// devela::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/num/float/_reexport.rs

mod ext_float; // FloatExt

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            ext_float::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;

        #[doc(inline)]
        pub use devela_base_core::num::{
            FloatConst,
            f32bits, f32bits_niche, f64bits, f64bits_niche, fsize,
        };

        crate::cfg_if! { if #[cfg(feature = "std")] {
            pub use devela_base_std::num::FloatStd as Float;
        } else {
            pub use devela_base_core::num::Float;
        }}
    }
}
