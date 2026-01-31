// devela::num::dom::real::float
//
#![doc = crate::_DOC_NUM_DOM_REAL_FLOAT!()] // private
#![doc = crate::_doc!(modules: crate::num::dom::real; float)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/num/dom/real/float/_reexport.rs

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
        pub use devela_base_core::num::dom::{ // real
            FloatConst,
            f32bits, f32bits_niche, f64bits, f64bits_niche, fsize,
        };

        crate::cfg_if! { if #[cfg(feature = "std")] {
            pub use devela_base_std::num::dom::{ // real
                FloatStd as Float,
            };
        } else {
            pub use devela_base_core::num::dom::{ // real
                Float,
            };
        }}
    }
}
