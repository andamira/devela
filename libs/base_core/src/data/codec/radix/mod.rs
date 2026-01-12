// devela_base_core::data::codec::radix
//
//! Radix-based encodings.
//

#[cfg(test)]
mod tests;

mod base; // Base*

crate::structural_mods! { // _mods
    _mods {
        pub use super::base::*;
    }
}
