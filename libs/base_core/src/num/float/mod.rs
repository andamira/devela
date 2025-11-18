// devela_base_core::num::float
//
#![doc = crate::_DOC_NUM_FLOAT!()]
//

mod alias; // fsize
mod bits; // f32bits, f64bits

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            alias::*,
            bits::*,
        };
    }
}
