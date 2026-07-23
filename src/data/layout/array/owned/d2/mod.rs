// devela/src/data/layout/array/owned/d2/mod.rs
//
//! 2-dimensional array
//

#[cfg(test)]
mod _test;

mod impl_traits;
mod methods;

mod define; // Array2d

crate::structural_mods! { // _mod
    _mods {
        pub use super::define::*;
    }
}
