// devela::num::wide
//
#![doc = crate::_DOC_NUM_WIDE!()]
//!
#![doc = crate::_doc!(extends: simd)]
//

mod _reexport_core; // SYMLINK to /libs/base_core/src/num/wide/_reexport.rs

crate::structural_mods! { // _reexports, _hidden
    _reexports {
        pub use super::_reexport_core::*;

        #[doc(inline)]
        pub use devela_base_core::num::wide::define_lane;
        #[cfg(feature = "_docs_min")]
        pub use devela_base_core::num::wide::Lane4_i32Example;
    }
    _hidden {
        pub use devela_base_core::{
            _dep_wide_compile, _dep_wide_use
        };
    }
}
