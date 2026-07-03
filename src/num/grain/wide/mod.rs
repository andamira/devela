// devela/src/num/grain/wide/mod.rs
//
#![doc = crate::_DOC_NUM_GRAIN_WIDE!()] // public
#![doc = crate::_doc!(modules: crate::num::grain; wide)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: simd)]
//

mod _reexport_core;

#[cfg(test)]
mod _test;

#[cfg(feature = "_docs_examples")]
mod _example;

mod _docs;
mod _helper; // __lane_dispatch!, _dep_wide_compile!, _dep_wide_use!

mod lane; // lane!

crate::structural_mods! { // _reexports, _hidden
    _mods {
        pub use super::lane::*;

        #[cfg(feature = "_docs_examples")]
        pub use super::_example::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
    _hidden {
        pub use super::{
            _docs::*,
            _helper::*,
        };
    }
}
