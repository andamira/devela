// devela/src/num/quant/scale/mod.rs
//
#![doc = crate::_DOC_NUM_QUANT_SCALE!()] // private
#![doc = crate::_doc!(modules: crate::num::quant; scale)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//

#[cfg(test)]
mod _test;

mod namespace; // Scale
// mod composition; // Vernier…

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::*,
        };
    }
}
