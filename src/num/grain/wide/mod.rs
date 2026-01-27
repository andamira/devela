// devela::num::grain::wide
//
#![doc = crate::_DOC_NUM_GRAIN_WIDE!()]
//!
#![doc = crate::_doc!(extends: simd)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/num/grain/wide/_reexport.rs

crate::structural_mods! { // _reexports, _hidden
    _reexports {
        pub use super::_reexport_core::*;

        #[doc(inline)]
        pub use devela_base_core::num::grain::define_lane;
        #[cfg(feature = "_docs_min")]
        pub use devela_base_core::num::grain::Lane4_i32Example;
    }
    _hidden {
        // hidden re-exports
        pub use devela_base_core::num::grain::{ // wide
            _dep_wide_compile, _dep_wide_use
        };
    }
}
