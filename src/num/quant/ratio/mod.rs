// devela/src/num/quant/ratio/mod.rs
//
//!
//

#[cfg(test)]
mod _test;

mod define; // Ratio

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
