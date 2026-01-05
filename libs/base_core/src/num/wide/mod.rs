// devela_base_core::num::wide
//
#![doc = crate::_DOC_NUM_WIDE!()]
//!
#![doc = crate::_doc!(extends: simd)]
//

mod _docs;
mod _helpers; // __lane_dispatch!, _dep_wide_compile!, _dep_wide_use!
mod _reexport; // SYMLINK from /src/num/wide/_reexport_core.rs

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
