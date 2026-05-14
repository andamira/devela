// devela::data::codec::radix
//
//! Radix-based encodings.
//

#[cfg(test)]
mod tests;

mod base; // Base*
// mod new_base; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            base::*,
            // new_base::*,
        };
    }
}
