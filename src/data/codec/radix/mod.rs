// devela::data::codec::radix
//
//! Radix-based encodings.
//

#[cfg(test)]
mod tests;

mod base; // Base*
// mod ext; // BaseExt WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            base::*,
            // ext::*,
        };
    }
}
