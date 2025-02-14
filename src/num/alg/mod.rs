// devela::num::algebra
//
//! Linear algebra and symbolic computation.
//

pub mod linear;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::linear::_all::*;

        // WIPZONE
        // pub use super::symb::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// pub mod symb;
