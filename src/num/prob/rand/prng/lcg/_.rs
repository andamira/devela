// devela/src/num/prob/rand/prng/lcg/_.rs
//
#![doc = concat![crate::_ABBR_LCG!(), "s."]]
//

crate::mods_in! {
    mod u16;
    // mod u32;
    // mod u64;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            u16::Lcg16,
            // u32::Lcg32,
            // u64::Lcg64,
        };
    }
}
