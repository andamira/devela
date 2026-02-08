// devela_base_core::num::fin::ord::cmp
//
//! Items to help comparing.
//

#[cfg(test)]
mod tests;

mod macros; // cmp!
mod wrapper; // Cmp

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            macros::*,
            wrapper::*,
        };
    }
}
