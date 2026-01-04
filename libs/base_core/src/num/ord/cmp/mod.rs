// devela_base_core::num::ord::cmp
//
//! Items to help comparing.
//

#[cfg(test)]
mod tests;

mod wrapper; // Cmp

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            wrapper::*,
        };
    }
}
