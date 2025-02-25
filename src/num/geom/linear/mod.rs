// devela::num::geom::linear
//
//! Linear algebra.
//

mod vector;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::vector::*;
        // WIPZONE
        // pub use super::matrix::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod matrix;
