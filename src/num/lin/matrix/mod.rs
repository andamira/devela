// devela::num::lin::matrix
//
//! Matrices.
//

mod definitions;
mod methods;

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definitions::*,
        };
    }
}
