// devela_base_core::num::quant::power
//
#![doc = crate::_DOC_NUM_QUANT_POWER!()] // MAYBE public
#![doc = crate::_doc!(modules: crate::num::quant; power)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//
// TODO: THINK as views over a scale as well.. and T could be... the base

// maybe also as functions… that can be generic… and therefore...copy const
// and have methods like simd over lanes... partial trait parallel functionality…
mod log; // WIP Log
mod power; // WIP Power
mod root; // WIP Root
mod tripow; // WIP TriPow

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            log::*,
            power::*,
            root::*,
            tripow::*,
        };
    }
}
