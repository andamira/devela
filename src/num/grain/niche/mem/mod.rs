// devela/src/num/grain/niche/mem/mod.rs
//
//! Numeric types for memory-efficient representations.
//

#[cfg(test)]
mod _test;

mod non_value; // NonValue*, NonMax*, NonMin*

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            non_value::*,
        };
    }
}
