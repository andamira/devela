// devela::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

mod ext_float; // FloatExt

// re-exports
crate::mod_path!(_c "../../../libs/base_core/src/num/float/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ext_float::*,
        };

        // re-exports
        pub use super::_c::*;
        #[doc(inline)]
        pub use devela_base_core::num::{
            FloatConst,
            f32bits, f32bits_niche, f64bits, f64bits_niche, fsize,
        };

        #[cfg(not(feature = "std"))]
        #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature = "std"))))]
        pub use devela_base_core::num::Float;

        #[cfg(feature = "std")]
        #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature = "std"))))]
        pub use devela_base_std::num::FloatStd as Float;
    }
}
