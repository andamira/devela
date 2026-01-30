// devela_base_core::num::prob::rand::pcg
//
#![doc = concat![crate::_ABBR_PCG!(), "s."]]
//

mod consts;

// mod u8; // Pcg8
// mod u16; // Pcg16
mod u32; // Pcg32
// mod u64; // Pcg64
// mod u128; // Pcg128

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // u8::*,
            // u16::*,
            u32::*,
            // u64::*,
            // u128::*,
        };
    }
    _crate_internals {
        pub(crate) use super::consts::*;
    }
}
