// devela::geom::linear
//
//! Linear algebra.
//

mod vector;
mod matrix;

// WIPZONE
// mod affine; // Affine

crate::structural_mods! { // _mods
    _mods {
        pub use super::vector::*;
        pub use super::matrix::*;

        // WIPZONE
        // pub use super::affine::*;
    }
}
