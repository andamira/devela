// devela::num::wide
//
#![doc = crate::_DOC_NUM_WIDE!()]
//!
#![doc = crate::_doc!(extends: simd)]
//

// re-exports
crate::mod_path!(_c "../../../libs/base_core/src/num/wide/reexports.rs");

crate::structural_mods! { // _hidden
    _mods {
        // re-exports
        pub use super::_c::*;
        #[doc(inline)]
        pub use devela_base_core::num::define_lane;
        #[cfg(feature = "_docs_min")]
        pub use devela_base_core::num::ExampleLane4_i32;
    }
    _hidden {
        pub use devela_base_core::{
            _dep_wide_compile, _dep_wide_use
        };
    }
}
