// devela_base_core::num::prob::rand::pcg
//
#![doc = concat![crate::_ABBR_PCG!(), "s."]]
//

mod define; // define_pcg!

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        define_pcg![pub Pcg32: (u32)];

        pub use super::{
            define::*,
        };
    }
}
