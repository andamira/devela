// devela::num::geom::linear
//
//! Linear algebra.
//

mod vector;
mod matrix;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::vector::*;
        pub use super::matrix::*;
        // WIPZONE
        // pub use super::affine::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod affine; // Affine
