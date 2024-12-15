// devela::num::alg::linear
//
//! Linear algebra.
//

// mod matrix; // TODO
mod vector;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::vector::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
