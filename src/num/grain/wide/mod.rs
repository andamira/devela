// devela::num::grain::wide
//
#![doc = crate::_DOC_NUM_GRAIN_WIDE!()] // public
#![doc = crate::_doc!(modules: crate::num::grain; wide)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: simd)]
//

mod _reexport_core;

mod _docs;
mod _helpers; // __lane_dispatch!, _dep_wide_compile!, _dep_wide_use!

#[cfg(test)]
mod tests;

mod lane; // define_lane!

crate::structural_mods! { // _reexports, _hidden
    _mods {
        pub use super::{
            lane::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
    _hidden {
        pub use super::{
            _docs::*,
            _helpers::*,
        };
    }
}
