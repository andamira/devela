// devela/src/num/lin/matrix/mod.rs
//
//! Matrices.
//

#[cfg(test)]
mod _test;

mod define; // Matrix
mod methods;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
