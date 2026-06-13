// devela/src/num/lin/matrix/mod.rs
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
