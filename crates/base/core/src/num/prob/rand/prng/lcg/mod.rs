// devela_base_core::num::prob::rand::lcg
//
#![doc = concat![crate::_ABBR_LCG!(), "s."]]
//

mod u16;
// mod u32;
// mod u64;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            u16::Lcg16,
            // u32::Lcg32,
            // u64::Lcg64,
        };
    }
}
