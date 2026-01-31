// devela_base_core::num::grain::wide
//
#![doc = crate::_DOC_NUM_GRAIN_WIDE!()] // public
#![doc = crate::_doc!(modules: crate::num::grain; wide)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: simd)]
//

mod _docs;
mod _helpers; // __lane_dispatch!, _dep_wide_compile!, _dep_wide_use!
mod _reexport; // SYMLINK from /src/num/grain/wide/_reexport_core.rs

#[cfg(test)]
mod tests;

mod lane; // define_lane!

crate::structural_mods! { // _mods, _reexports, _hidden
    _mods {
        pub use super::{
            lane::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
    _hidden {
        pub use super::{
            _docs::*,
            _helpers::*,
        };
    }
}
